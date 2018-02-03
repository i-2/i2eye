## i2-EYE

A simple webservice that gives out the text from image url.

## Supported Formats

* JPG
* PNG
* GIF
* TIFF

## Develop

You could always use the virtual machine to develop. I have added a vagrant file
with necessary configuration to help with development.

```shell

vagrant up
vagrant ssh

cd /vagrant

# to test
RUST_LOG=debug cargo test -- --nocapture

```

## API

* ```/?q=<imagetosearch>``` - Make a get request with image url as search string.

## Example

```
curl -v http://localhost:8080/?q=https://i.stack.imgur.com/t3qWG.png
```

## LICENSE
MIT