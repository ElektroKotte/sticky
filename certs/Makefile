.PHONY: all clean cleaner verify

HOST ?= kottland.net
CLIENT ?= $(shell hostname)
ISSUER ?= Kotte

all: certs
certs: server.pem client-$(CLIENT).crt

clean:
	rm -f server-* client-*

cleaner: clean
	rm -f ca.*


verify: ca.crt server-$(HOST).crt client-$(CLIENT).crt
	openssl verify -CAfile $< server-$(HOST).crt
	openssl verify -CAfile $< client-$(CLIENT).crt

#### File creation ####

ca.key server-$(HOST).key client-$(CLIENT).key:
	openssl genrsa -out $@ 4096

ca.crt: ca.key
	openssl req -new -x509 -days 3650 -key $< -out $@ -subj "/O=$(ISSUER)/"

%.csr: %.key
	openssl req -new -key $< -out $@ -subj "/CN=$(HOST)/"

%.crt: %.csr ca.crt ca.key
	openssl x509 -req -CAcreateserial -days 365 -in $< -CA ca.crt -CAkey ca.key -out $@

server.pem: server-$(HOST).crt server-$(HOST).key
	cat $^ > $@
