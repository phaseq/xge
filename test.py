import socket

s = socket.socket(
    socket.AF_INET, socket.SOCK_STREAM)

s.connect(("127.0.0.1", 7787))
for i in range(20):
	#s.send('a' * (i + 1) + '\n')
	s.send('["/Users/fabian/Dev/Rust/xge-wrap/target/debug",'
		'"/Users/fabian/Dev/Rust/xge-wrap/target/debug/xge-wrap","0",'
		'"/usr/local/bin/python","-c","print \'hi\'"]\n')
s.close()