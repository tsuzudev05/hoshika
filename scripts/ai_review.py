import os
import json
import sys
from pathlib import Path
from groq import Groq
import urllib.request
import urllib.error

MONTHLY_USAGE_FILE = ".monthly-usage.json"
MAX_DIFF_CHARS = 60000
MODEL = "llama-3.3-70b-versatile"


def load_monthly_usage():
    if Path(MONTHLY_USAGE_FILE).exists():
        with open(MONTHLY_USAGE_FILE) as f:
            return json.load(f)
    return {"total_tokens": 0, "runs": 0}


def save_monthly_usage(usage):
    with open(MONTHLY_USAGE_FILE, "w") as f:
        json.dump(usage, f)


def post_pr_comment(token, repo, pr_number, body):
    if not pr_number:
        print("No PR number, skipping comment")
        return
    url = f"https://api.github.com/repos/{repo}/issues/{pr_number}/comments"
    data = json.dumps({"body": body}).encode()
    req = urllib.request.Request(url, data=data, method="POST")
    req.add_header("Authorization", f"Bearer {token}")
    req.add_header("Content-Type", "application/json")
    req.add_header("Accept", "application/vnd.github+json")
    with urllib.request.urlopen(req) as resp:
        print(f"Comment posted: {resp.status}")


def main():
    groq_api_key = os.environ["GROQ_API_KEY"]
    github_token = os.environ["GITHUB_TOKEN"]
    pr_number = os.environ.get("PR_NUMBER", "")
    repo = os.environ.get("REPO", "")
    pr_title = os.environ.get("PR_TITLE", "")
    base_branch = os.environ.get("BASE_BRANCH", "main")
    head_branch = os.environ.get("HEAD_BRANCH", "")

    diff = Path("diff.txt").read_text(encoding="utf-8", errors="replace")
    if not diff.strip():
        print("No diff found, skipping review")
        return

    if len(diff) > MAX_DIFF_CHARS:
        diff = diff[:MAX_DIFF_CHARS] + "\n\n... (diff truncated)"

    client = Groq(api_key=groq_api_key)

    prompt = f"""あなたは熟練したコードレビュアーです。以下のgit diffをレビューしてください。

PR タイトル: {pr_title}
ブランチ: {head_branch} → {base_branch}

以下の観点でレビューしてください：
- バグ・正確性の問題
- セキュリティの脆弱性
- パフォーマンスの問題
- コードの可読性・保守性
- エッジケースの考慮漏れ

指摘は日本語で、簡潔かつ具体的に記述してください。問題がなければその旨も明記してください。

Diff:
```diff
{diff}
```"""

    response = client.chat.completions.create(
        model=MODEL,
        messages=[{"role": "user", "content": prompt}],
        max_tokens=2000,
    )

    review_text = response.choices[0].message.content
    total_tokens = response.usage.total_tokens

    usage = load_monthly_usage()
    usage["total_tokens"] += total_tokens
    usage["runs"] += 1
    save_monthly_usage(usage)
    print(f"Tokens used: {total_tokens} (monthly total: {usage['total_tokens']})")

    comment_body = (
        f"## 🤖 AI Code Review\n\n"
        f"{review_text}\n\n"
        f"---\n"
        f"*Powered by Groq ({MODEL}) · 今月の累計トークン: {usage['total_tokens']:,}*"
    )

    post_pr_comment(github_token, repo, pr_number, comment_body)


if __name__ == "__main__":
    main()
