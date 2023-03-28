import json

with open('postman.json') as f:
    data = json.load(f)

for item in data['item']:
    request = item['request']
    method = request['method']
    url = request['url']
    headers = request['header']
    body = request['body']
    raw_body = None

    # 如果请求正文为raw模式，提取raw正文
    if body['mode'] == 'raw':
        raw_body = body['raw']

    print('###')

    # 构造HTTP请求
    print(f"{method} {url}")
    print()
    for header in headers:
        print(f"{header['key']}: {header['value']}")
    if raw_body is not None:
        print(raw_body)
