# RNADNADecoderRust

**RNADNADecoderRust** (in short: Decoder) is program for converting RNA sequences into
aminoacids chains

Things to do
- Write conversion from DNA to RNA
- Write Dockerfile for application

---

These are steps for running project:
```
git clone https://github.com/PiotrTrybala/RNADNADecoderRust.git
cd RNADNADecoderRust
cargo build && cargo run
```
## API

To access api on port 8000 (or any specified) by url:
```plaintext
<your ip address>:8000/api/v1/decoder
```

and you have to provide json body:
```json
{
  "input": "<your RNA or DNA sequence>"
}
```
---

## Docker

To use docker, first you have to build the dockerfile.

For that command is:

For linux: ```sudo docker -t decoder:latest .```<br/>
For Windows or Mac: ```to be provided```

Then after building, to run the container:

For linux: ```sudo docker run -p 5000:8000 -d --name "decoder" --ip 172.17.0.3 decoder:latest```<br/>
For Windows or Mac: ```to be provided```
