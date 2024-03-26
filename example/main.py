from context import Context
import os
from openai import OpenAI
import json

client = OpenAI(
    api_key=os.environ.get("OPENAI_API_KEY"),
)

def main():

    with Context("http://localhost:3002") as ctx:

        while True:
            history = ctx.get_terminal_history()
            
            prompt = input("> ")
            cmd = call_llm(history=history, prompt=prompt)
            
            ctx.exec(cmd)


def call_llm(history: str, prompt: str) -> str:

    completion = client.chat.completions.create(
        response_format={ "type": "json_object" },
        messages=[
            {"role": "system", "content": f"You are a terminal shell, you can only output commands and code. This is the history of previous commands in the terminal: {history}."},
            {"role": "user", "content": f"You have been given the task: {prompt}\n. Respond with a json containing only one key called `command` and with the value being only the command as a string. Given the task, your command is:"},
        ],
        model="gpt-4-turbo-preview",
    )

    res = json.loads(completion.choices[0].message.content)

    return res['command']


if __name__ == "__main__":
    main()