import html
import json
import os
import re
import urllib.request
from collections import defaultdict
from datetime import datetime

import bleach
import markdown

REPO = os.environ["REPO"]
TOKEN = os.environ["GH_TOKEN"]
MODULES = [
    module.strip()
    for module in os.environ.get("MODULES", "").split(",")
    if module.strip()
]

API = f"https://api.github.com/repos/{REPO}"

HEADERS = {
    "Authorization": f"Bearer {TOKEN}",
    "Accept": "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
}

BRANCH_PATTERN = re.compile(r"^(.+?)-(feat|fix|[a-z]+)-(.+)$")

def branch_url(branch):
    from urllib.parse import quote

    return f"https://github.com/{REPO}/tree/{quote(branch, safe='/')}"

def render_markdown(text):
    """
    Convert GitHub PR Markdown into safe HTML.
    """

    if not text:
        text = "No description provided."

    rendered = markdown.markdown(
        text,
        extensions=[
            "fenced_code",
            "tables",
            "nl2br",
            "sane_lists",
            "toc",
        ],
    )

    allowed_tags = set(bleach.sanitizer.ALLOWED_TAGS).union(
        {
            "p",
            "pre",
            "code",
            "h1",
            "h2",
            "h3",
            "h4",
            "hr",
            "br",
            "div",
            "span",
            "table",
            "thead",
            "tbody",
            "tr",
            "th",
            "td",
            "ul",
            "ol",
            "li",
            "blockquote",
            "del",
            "details",
            "summary",
        }
    )

    allowed_attributes = {
        "a": ["href", "title", "rel"],
        "code": ["class"],
        "pre": ["class"],
        "th": ["align"],
        "td": ["align"],
    }

    return bleach.clean(
        rendered,
        tags=allowed_tags,
        attributes=allowed_attributes,
        protocols=["http", "https", "mailto"],
        strip=True,
    )


def github_get(url):
    request = urllib.request.Request(url, headers=HEADERS)

    with urllib.request.urlopen(request) as response:
        return json.loads(response.read())


def get_all_pull_requests():
    pull_requests = []
    page = 1

    while True:
        url = f"{API}/pulls?state=all&per_page=100&page={page}"

        batch = github_get(url)

        if not batch:
            break

        pull_requests.extend(batch)

        if len(batch) < 100:
            break

        page += 1

    return pull_requests


def parse_branch(branch):
    match = BRANCH_PATTERN.match(branch)

    if not match:
        return None

    return {
        "module": match.group(1),
        "type": match.group(2),
        "feature": match.group(3),
    }


def format_date(date):
    if not date:
        return "Unknown"

    return datetime.fromisoformat(date.replace("Z", "+00:00")).strftime("%Y-%m-%d")


def date_key(item):
    return item.get("date") or ""


def generate_dashboard(pull_requests):
    modules = defaultdict(lambda: defaultdict(list))

    for pr in pull_requests:
        branch = pr["head"]["ref"]

        parsed = parse_branch(branch)

        if not parsed:
            continue

        module = parsed["module"]

        # Filter modules
        if MODULES and module not in MODULES:
            continue

        branch_type = parsed["type"]

        modules[module][branch_type].append(
            {
                "branch": branch,
                "feature": parsed["feature"],
                "title": pr["title"],
                "number": pr["number"],
                "url": pr["html_url"],
                "author": pr["user"]["login"],
                "description": pr.get("body") or "No description provided.",
                "state": ("merged" if pr["merged_at"] else pr["state"]),
                "date": pr["created_at"],
                "merged_at": pr["merged_at"],
            }
        )

    output = []

    # Show all requested modules, even if they have no PRs
    module_names = MODULES if MODULES else sorted(modules)

    for module in module_names:
        branches = modules[module]

        output.append(f"""
        <section class="module">
            <h2>{html.escape(module)}</h2>

            <div class="latest">
                {render_latest(branches, "feat")}
                {render_latest(branches, "fix")}
                {render_most_active_branch(pull_requests, module)}
            </div>

            <h3>PR history</h3>

            {render_history(branches)}
        </section>
        """)

    return "".join(output)


def render_latest(branches, branch_type):

    entries = branches.get(branch_type, [])

    if not entries:
        return f"""
        <div class="latest-card">

            <small>LATEST {branch_type.upper()}</small>

            <p>No branches found</p>

        </div>
        """

    latest = max(entries, key=date_key)

    description = render_markdown(latest.get("description", ""))

    return f"""
    <div class="latest-card">

        <small>LATEST {branch_type.upper()}</small>

        <h3>
            <a href="{html.escape(branch_url(latest['branch']), quote=True)}"
               target="_blank"
               rel="noopener noreferrer">
                {html.escape(latest['feature'])}
            </a>
        </h3>

        <a href="{html.escape(latest["url"])}"
           target="_blank"
           rel="noopener noreferrer">

            PR #{latest["number"]}

        </a>

        <p>{format_date(latest["date"])}</p>

        <p>Author: {html.escape(latest["author"])}</p>

        <span class="status {html.escape(latest["state"])}">
            {html.escape(latest["state"])}
        </span>

        <details class="pr-description">

            <summary>Show PR description</summary>

            <div class="description-content">
                {description}
            </div>

        </details>

    </div>
    """


def render_most_active_branch(pull_requests, module):

    branch_counts = defaultdict(int)

    for pr in pull_requests:
        target_branch = pr["base"]["ref"]

        parsed = parse_branch(target_branch)

        if not parsed:
            continue

        if parsed["module"] != module:
            continue

        # Count only merged PRs
        if pr["merged_at"]:
            branch_counts[target_branch] += 1

    if not branch_counts:
        return """
        <div class="latest-card">
            <small>MOST ACTIVE BRANCH</small>
            <p>No merged PRs found</p>
        </div>
        """

    most_active = max(branch_counts, key=lambda branch: branch_counts[branch])

    count = branch_counts[most_active]

    return f"""
    <div class="latest-card">

        <small>MOST ACTIVE BRANCH / MAIN BRANCH</small>

        <h3>
            <a href="{html.escape(branch_url(most_active), quote=True)}"
               target="_blank"
               rel="noopener noreferrer">
                {html.escape(most_active)}
            </a>
        </h3>

        <p>{count} merged PR(s)</p>

    </div>
    """


def render_history(branches):

    all_entries = []

    for entries in branches.values():
        all_entries.extend(entries)

    all_entries.sort(key=date_key, reverse=True)

    if not all_entries:
        return "<p>No pull requests found for this module.</p>"

    def render_rows(entries):

        rows = []

        for item in entries:
            description = render_markdown(item.get("description", ""))

            rows.append(f"""
            <tr>

                <td>
                    <a href="{html.escape(branch_url(item['branch']), quote=True)}"
                       target="_blank"
                       rel="noopener noreferrer">
                        <code>{html.escape(item['branch'])}</code>
                    </a>
                </td>

                <td>
                    <a href="{html.escape(item["url"])}"
                       target="_blank"
                       rel="noopener noreferrer">
                        #{item["number"]} - {html.escape(item["title"])}
                    </a>

                    <details class="pr-description">
                        <summary>Show PR description</summary>

                        <div class="description-content">
                            {description}
                        </div>
                    </details>
                </td>

                <td>{html.escape(item["author"])}</td>

                <td>
                    <span class="status {html.escape(item["state"])}">
                        {html.escape(item["state"])}
                    </span>
                </td>

                <td>{format_date(item["date"])}</td>

            </tr>
            """)

        return "".join(rows)

    def render_table(entries):

        return f"""
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th>Branch</th>
                        <th>Pull Request</th>
                        <th>Author</th>
                        <th>Status</th>
                        <th>Created</th>
                    </tr>
                </thead>

                <tbody>
                    {render_rows(entries)}
                </tbody>
            </table>
        </div>
        """

    latest_entries = all_entries[:3]
    remaining_entries = all_entries[3:]

    history_html = render_table(latest_entries)

    if remaining_entries:
        history_html += f"""
        <details class="history-expand">

            <summary>
                Show all {len(all_entries)} pull requests
                ({len(remaining_entries)} more)
            </summary>

            {render_table(remaining_entries)}

        </details>
        """

    return history_html


def main():
    pull_requests = get_all_pull_requests()

    dashboard = generate_dashboard(pull_requests)

    os.makedirs("dist", exist_ok=True)

    with open("dist/index.html", "w") as file:
        file.write(f"""
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport"
                  content="width=device-width, initial-scale=1">

            <title>Module Dashboard</title>

            <style>
                body {{
                    font-family: Arial, sans-serif;
                    background: #f5f6fa;
                    color: #222;
                    margin: 0;
                    padding: 30px;
                }}

                h1 {{
                    margin-bottom: 30px;
                }}

                .module {{
                    background: white;
                    padding: 24px;
                    margin-bottom: 25px;
                    border-radius: 12px;
                    box-shadow: 0 2px 8px #0001;
                }}

                .latest {{
                    display: grid;
                    grid-template-columns:
                        repeat(auto-fit, minmax(200px, 1fr));
                    gap: 15px;
                }}

                .latest-card {{
                    background: #f5f6fa;
                    padding: 15px;
                    border-radius: 8px;
                }}

                .latest-card small {{
                    color: #666;
                    font-weight: bold;
                }}

                table {{
                    width: 100%;
                    border-collapse: collapse;
                    margin-top: 15px;
                }}

                th, td {{
                    padding: 12px;
                    text-align: left;
                    border-bottom: 1px solid #eee;
                }}

                a {{
                    color: #2563eb;
                    text-decoration: none;
                }}

                @media (max-width: 600px) {{
                    body {{ padding: 12px; }}
                    table {{ font-size: 12px; }}
                }}

                .pr-description {{
                    margin-top: 8px;
                    font-size: 12px;
                }}

                .pr-description summary {{
                    cursor: pointer;
                    color: #64748b;
                    user-select: none;
                    padding: 4px 0;
                    }}

                .pr-description summary:hover {{
                    color: #2563eb;
                }}

                .description-content {{
                    margin-top: 8px;
                    padding: 12px;
                    background: #f8fafc;
                    border: 1px solid #e2e8f0;
                    border-radius: 6px;
                    color: #334155;
                    white-space: pre-wrap;
                    overflow-wrap: anywhere;
                    line-height: 1.5;
                    max-height: 300px;
                    overflow-y: auto;
                    }}

                    .history-expand {{
                        margin-top: 15px;
                        }}

                    .history-expand summary {{
                        cursor: pointer;
                        color: #2563eb;
                        font-weight: bold;
                        padding: 12px;
                        background: #f5f6fa;
                        border-radius: 8px;
                        user-select: none;
                        }}

                    .history-expand summary:hover {{
                        background: #eef2ff;
                    }}

                    .history-expand[open] summary {{
                        margin-bottom: 10px;
                        }}
            </style>
        </head>

        <body>
            <h1>Module Development Progress Dashboard</h1>

            {dashboard}

        </body>
        </html>
        """)

    print("Dashboard generated successfully!")


if __name__ == "__main__":
    main()
