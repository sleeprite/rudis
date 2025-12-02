import socket

def test_info_server_command():
    # Connect to the Rudis server
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect(('localhost', 6379))
        
        # Send INFO server command
        sock.send(b'*2\r\n$4\r\nINFO\r\n$6\r\nserver\r\n')
        
        # Receive response
        response = sock.recv(4096)
        print("INFO server command response:")
        print(response.decode('utf-8'))
        
    finally:
        sock.close()

if __name__ == "__main__":
    test_info_server_command()