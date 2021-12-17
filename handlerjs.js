function fibonacci(n) {
  if (n < 2){
      return 1;
  } else {
      return fibonacci(n-1) + fibonacci(n-2);
  }
}

module.exports.hello = async (event) => {
    const num = parseInt(event.queryStringParameters.num)
    const result = fibonacci(num)
    return {
      statusCode: 200,
      headers: {
        'Content-type': 'application/json'
      },
      body: JSON.stringify(
        {
          result
        },
        null,
        2
      ),
    };
  };