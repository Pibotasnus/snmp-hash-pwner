# SNPM HASH PWNER

## Introduction

This is a small utility to crack a password from an SNMP authentication exchange.

## How to use it

It is fairly easy. You will have to extract three components from the network capture:

- [x] **Message**: The entirety of the message send by the client to the server. The message should be in hex string format.
- [x] **Hash / signature**: The result of the signature calculation
- [x] **Engine ID**: The engine id of the SNMP Agent.

Write it all to a file using this format:

```
<message>:<hash>:<engine-id>
```

Use your a wordlist for your dictionnary attack

```shell
snmphashpwner -d wordlist.txt -t target.txt
```

## Limitations

Works only for MD5
