import json
import subprocess
import time

def send_message(proc, method, params=None):
    message = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params or {}
    }
    content = json.dumps(message)
    length = len(content)
    proc.stdin.write(f"Content-Length: {length}\r\n\r\n{content}".encode())
    proc.stdin.flush()

def read_message(proc):
    headers = {}
    while True:
        line = proc.stdout.readline().decode().strip()
        if line == "":
            break

        print(line)
        key, value = line.split(": ")
        headers[key] = value
    
    if "Content-Length" in headers:
        length = int(headers["Content-Length"])
        content = proc.stdout.read(length).decode()
        return json.loads(content)

# Start your LSP server
lsp_server = subprocess.Popen(["target/debug/gen-lsp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)

# Initialize the server
send_message(lsp_server, "initialize", {
    "processId": None,
    "rootUri": "file:///Users/pepe/projects/github.com/pepegar/gen-lsp",
    "capabilities": {}
})

response = read_message(lsp_server)
print("Initialize response:", json.dumps(response, indent=2))

while True:
    response = read_message(lsp_server)
    print(json.dumps(response, indent=2))
    time.sleep(0.2)
