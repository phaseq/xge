import socket
import threading
import subprocess

XGE_LOCATION="target/release/xge"

s = socket.socket(
    socket.AF_INET, socket.SOCK_STREAM)

s.bind(("127.0.0.1", 0))
port = s.getsockname()[1]
address_string = "127.0.0.1:{}".format(port)
print "listening..."
s.listen(1)
print address_string

#xge_lock = threading.Lock()
#xge_lock.acquire()

xge_client = threading.Thread(target=lambda: 
	subprocess.call([XGE_LOCATION, address_string])
	)
xge_client.start()

print "thread started"

(cs, address) = s.accept()

print "client connected: {}".format(address)

#s.connect(("127.0.0.1", 7787))
for i in range(20):
	#s.send('a' * (i + 1) + '\n')
	cs.send('["sometitle","/Users/fabian/Dev/Rust/xge/target/release",'
		'"/Users/fabian/Dev/Rust/xge/target/release/xge-wrap","{}",'
		'"/usr/local/bin/python","-c","print \'hi\'"]\n'.format(i))
cs.close()