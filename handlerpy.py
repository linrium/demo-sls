import json


def fibonacci(n):
   if n <= 1:
       return n
   else:
       return(fibonacci(n-1) + fibonacci(n-2))


def hello(event, context):
    num = int(event['queryStringParameters']['num'])
    result = fibonacci(num)
    body = {
        "result": result,
    }

    response = {"statusCode": 200, "headers": { "Content-Type": "application/json" }, "body": json.dumps(body)}

    return response