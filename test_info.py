import socket

def test_info_command():
    # Connect to the Rudis server
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect(('localhost', 6379))
        
        # Send INFO command
        sock.send(b'*1\r\n$4\r\nINFO\r\n')
        
        # Receive response
        response = sock.recv(4096)
        print("INFO command response:")
        print(response.decode('utf-8'))
        
    finally:
        sock.close()

if __name__ == "__main__":
    test_info_command()