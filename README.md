# vlpkg
the soon to be package manager for veridion linux (my distro!!)
its written in rust and at this stage is an mvp!!
## features
- install from an archive
- removing a package
- listing installed packages
## how it works
vlpkg uses install and remove scripts to make porting packages alot more easier (or harder idk yet)
the package manager does not use repositories in a traditional sense now yes you can use them but all installs are from archives (.tar.zst) 
when you remove a package it simply just runs a remove script and removes the package from installed.log and when you install a package it unpacks the archive runs the install script and copies the remove script to /var/lib/vlpkg/remove/<package>/remove.sh i will remove hardcoded paths in the future but i just want something to show rn and logs the install in installed.log (just the name so no dupe installs etc) the package manager has to be ran as root (apoligise for the bad writing lol)
## how to install
to install vlpkg all you have to do is clone the repo

`git clone https://github.com/v1nch3ns0/vlpkg`

and then youll want to compile it

`cargo build --release`

and just move the binary to anywhere in path (/usr/bin recommended)

`sudo mv target/release/vlpkg`

and then all you have to do is init the directories!!

`sudo vlpkg init`

and its installed simple right?
### requirements for making packages
- it should be a tar archive with the zstandard compression method
- it should contain an install.sh file for installing the package and a remove.sh for removing the package
**that is it!!** ik crazy right its very easy to just throw in a binary and move it to /usr/bin (and of course your libs) but still its a very simple package manager

## notices
- its pretty cool but still very wip so think about that before you slander the whole thing bc it left ur system half broken lols
- its designed to be very simple so there wont be any dependency hell or anything like that i liked the idea of the kiss (keep it simple stupid) linux package manager but not to the extreme (if yk what i mean)
