/// 检查 GitHub Releases 最新版本
///
/// 使用 GitHub 免认证 API，每请求会返回最新的 Release（按 semver 倒序）。
/// 如果请求失败（无网、API 限速等），返回 None，由前端静默处理。

use serde::Deserialize;

/// GitHub Release API 返回的最外层对象
#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
  pub tag_name: String,
  pub html_url: String,
}

/// 检查更新的返回结果
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckResult {
  /// 是否有新版本
  pub has_update: bool,
  /// 最新版本 tag（如 v1.0.2），无网络时为 None
  pub latest_tag: Option<String>,
  /// 当前版本（编译时从 Cargo.toml 读取）
  pub current_version: String,
  /// Release 页面 URL
  pub release_url: Option<String>,
  /// 错误消息（可选）
  pub error: Option<String>,
}

/// 从 GitHub Releases API 取最新版本信息
pub fn check_latest_release() -> UpdateCheckResult {
  let current = env!("CARGO_PKG_VERSION").to_string();
  let url = "https://api.github.com/repos/zentrix566/my-pic-viewer/releases/latest";

  // ureq 要求设置 User-Agent，否则 GitHub 会 403
  let ua = "my-pic-viewer-updater/1.0";

  match ureq::get(url).set("User-Agent", ua).set("Accept", "application/json").call() {
    Ok(response) => {
      if response.status() != 200 {
        return UpdateCheckResult {
          has_update: false,
          latest_tag: None,
          current_version: current,
          release_url: None,
          error: Some(format!("GitHub API 返回状态码 {}", response.status())),
        };
      }

      match response.into_json::<GitHubRelease>() {
        Ok(release) => {
          let latest_tag = release.tag_name;
          let has_update = has_new_version(&current, &latest_tag);
          UpdateCheckResult {
            has_update,
            latest_tag: Some(latest_tag),
            current_version: current,
            release_url: Some(release.html_url),
            error: None,
          }
        }
        Err(e) => UpdateCheckResult {
          has_update: false,
          latest_tag: None,
          current_version: current,
          release_url: None,
          error: Some(format!("解析 JSON 失败: {}", e)),
        },
      }
    }
    Err(e) => UpdateCheckResult {
      has_update: false,
      latest_tag: None,
      current_version: current,
      release_url: None,
      error: Some(format!("网络请求失败: {}", e)),
    },
  }
}

/// 解析版本号，比较当前版本和最新 release tag 是否有更新
///
/// 处理 tag 格式：v1.0.2 -> (1, 0, 2)
/// 如果解析不成功，按字符串比较（偏保守，认为无更新）
fn has_new_version(current: &str, latest_tag: &str) -> bool {
  // 去掉最前面的 v
  let trimmed = latest_tag.trim_start_matches('v');

  // 去掉 pre-release 后缀，如 v1.0.3-beta -> 1.0.3
  let base = trimmed.split(|c: char| !c.is_ascii_digit() && c != '.').next().unwrap_or(trimmed);

  let parse_semver = |s: &str| -> Option<(u64, u64, u64)> {
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    if parts.len() == 3 {
      Some((parts[0].parse().ok()?, parts[1].parse().ok()?, parts[2].parse().ok()?))
    } else {
      None
    }
  };

  match (parse_semver(&current.replace('v', "")), parse_semver(base)) {
    (Some(cur), Some(latest)) => latest > cur,
    _ => {
      // 解析失败时回退到字符串比较，但大部分情况已覆盖
      latest_tag.trim_start_matches('v') > current
    }
  }
}
