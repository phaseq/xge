import socket
import threading
import subprocess
import json

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

results_condition = threading.Condition()
results = []

def xge_thread():
	proc = subprocess.Popen([XGE_LOCATION, "client", address_string], stdout=subprocess.PIPE)
	while True:
		line = proc.stdout.readline()
		if line == '': break
		line = line.split('mwt ')
		if len(line) == 1: continue
		result = json.loads(line[1])
		results_condition.acquire()
		results.append(result)
		results_condition.notify()
		results_condition.release()

xge_client = threading.Thread(target=xge_thread)
xge_client.start()

print "thread started"

(cs, address) = s.accept()

print "client connected: {}".format(address)

#s.connect(("127.0.0.1", 7787))
for i in range(20):
	#s.send('a' * (i + 1) + '\n')
	cs.send('["/Users/fabian/Dev/Rust/xge","sometitle","{}",'
		'"python","-c","print \'hi\'"]\n'.format(i))
cs.close()

while xge_client.is_alive():
	for result in results:
		print "-->", result
	with results_condition:
		results_condition.wait(timeout=360)