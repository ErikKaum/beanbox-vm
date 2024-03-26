# Example

To get your env set up run these:

```bash
python3 -m venv venv
source venv/bin/activate
pip3 install -r requirements.txt
```

We're also using a selenium chrome driver so you'll need to have Chrome installed. Alternatively you can swap it for something else.

Set your OpenAI api key and just run the main

```bash
export OPENAI_API_KEY="<YOUR KEY>"
python3 main.py
```

It opens the browser based terminal which is connected to the Beanbox microVM and should get a view like this in your terminal:
```bash
>
```
Just go ahead and prompt the llm.
Happy hacking!