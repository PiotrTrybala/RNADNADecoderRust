# RNADNADecoderRust

RNADNADecoderRust (in short: Decoder) is program for converting RNA sequences into
aminoacids chains

Things to do
- Write conversion from DNA to RNA
- Write Dockerfile for application

API

To access api on port 8000 (or any specified) by url

<your ip address>:8000/api/v1/decoder

and providing json body:

{
  "input": "<your RNA and DNA sequence>"
}

Docker

To use docker, first you have to build the dockerfile.

For that command is:

For linux: sudo docker -t decoder:latest .
For Windows or Mac: to be provided

Then after building, to run the container:

For linux: sudo docker run -p 5000:8000 -d --name "decoder" --ip 172.17.0.3 decoder:latest
For Windows or Ma: to be provided
