from ollama import Client
from ollama import ChatResponse

c = Client(
  host='http://BigChungus:11434',
  headers={'x-some-header': 'some-value'}
)

response: ChatResponse = c.chat(model='gemma3:12b', messages=[
  {
    'role': 'user',
    'content': 'Why is the sky blue?',
  },
])
print(response['message']['content'])
# or access fields directly from the response object
print(response.message.content)