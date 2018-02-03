# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|

  config.vm.box = "ubuntu/trusty64"
  config.vm.network "forwarded_port", guest: 3890 , host: 8080
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install build-essential
    apt-get install tesseract-ocr
    curl https://sh.rustup.rs -sSf | sh
  SHELL

end
