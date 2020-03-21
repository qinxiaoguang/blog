/**
 * file:path.rs
 * des:util about some path problem
 */

const URL_PATH_SPLIT: &str = "/";

// 是否能match上某个通配url
pub fn can_match(pattern: &str, s: &str) -> bool {
    // 分隔url及pattern
    // 使用dp的方式来判断是否可以match,
    // dp[i][j]:表示 pattern[0..i]与s[0..j]是否match.
    // 那么更新规则就是: if pattern[i] == "*": dp[i][j] = dp[i][j-1]|. if pattern[i] == s[j]; dp[i][j] = dp[i-1][j-1]
    let patterns: Vec<&str> = pattern.split(URL_PATH_SPLIT).collect();
    let ss: Vec<&str> = s.split(URL_PATH_SPLIT).collect();
    // 创建动态二维数组太扯了。
    let mut dp = vec![vec![false; ss.len() + 1]; patterns.len() + 1];
    dp[0][0] = true;
    for (i, p) in patterns.iter().enumerate() {
        for (j, sitem) in ss.iter().enumerate() {
            if j == 0 {
                dp[i + 1][j] = false;
            }
            dp[i + 1][j + 1] = if p == &"*" {
                dp[i + 1][j] || dp[i][j + 1] || dp[i][j]
            } else if &*p == sitem {
                dp[i][j]
            } else {
                false
            };
        }
    }
    dp[patterns.len()][ss.len()]
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_match() {
        println!("{}", super::can_match("/*", "/a"));
        assert_eq!(true, super::can_match("/*", "/a"));
        assert_eq!(true, super::can_match("/*", "/a"));
        assert_eq!(true, super::can_match("/*", "/a/b"));
        assert_eq!(false, super::can_match("/*/lala", "/a/b/haha"));
        assert_eq!(true, super::can_match("/admin/*", "/admin"));
        assert_eq!(false, super::can_match("/admin/*", "/auth/callback"));
        assert_eq!(true, super::can_match("/admin/*", "/admin/article/ping"));
    }

    #[test]
    pub fn test_mytest() {
        let res = (1..10).fold(0, |a, b| a + b);
        println!("{}", res);
    }
}
