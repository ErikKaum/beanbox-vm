# Beanbox VM
Beanbox is a microVM built for LLMs and humans. The core idea is to allow LLMs to control a computer in isolation and in a safe way. Additionally, the interaction with the LLM should be like pair coding. You can write a few commands and the LLM can continue from there.

The design principles of Beanbox are:
- Simple
    - one command `beanbox run`
    - the interaction should be as simple as sharing a terminal with anyone else
- Fast: boot times should be in the 100-200ms region
- Secure: both you and your LLM can go rouge in the Beanbox without causing harm to the host machine

> :warning: **Early days**: expect rough edges

# Run on your machine
I've so far only ran Beanbox on aarch64-apple-darwin but it should work on linux (x86_64) as well.

## tl;dr
There's currently no official release so the way to run this is by building from source. To do this:
Run `brew install libkrun podman just` and run `just build`.

## I wanna know more
Beanbox relies on the [libkrun](https://github.com/containers/libkrun) virtualisation framwork for creating VMs. On linux libkrun uses the KVM hypervisor and on macOS the Hypervisor.Framework. Libkrun is available on homebrew: `brew install libkrun`.

Next up you need to prepare a rootfs. You can in principle unpack any OCI container image and use that as the rootfs. There's a recipie in the justfile that relies on podman for unpacking the .tar and automatically sets the dns nameserver to 1.1.1. in resolv.conf. The default image used in ubuntu:latest.
Once you have podman installed run `just rootfs`.

Run `just build-gotty` which builds the fork of gotty and places the binary in the rootfs under `/bin/gotty`. Gotty is launced on VM startup and is what facilitates the terminal sharing between the host and guest.

Finally `just build-cli` builds the cli which manages the Beanbox VM. On macOS it sets the correct codesign entitlements as well.

You should now have a binary called beanbox in the root of the repo. Happy hacking!

# Usage

## In browser

Once the Beanbox is running you should see a message similar to this (port may vary):
![init-screen](https://github.com/erikkaum/beanbox/blob/main/readme-assets/init-screen?raw=true.png)

Open a browser on the assigned port and run commands in the Beanbox.
Like so:
![browswer](https://github.com/erikkaum/beanbox/blob/main/readme-assets/borwser?raw=true.png)


## Example with LLM 
For a more more smooth interaction, I've provided a simple app in `/example` that makes GPT-4 run code in Beanbox. This is the same code as in the demo video above.

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

