# murcott

## Objectives

### Context
I'm currently studying at ETML, a technical school in Lausanne.
We have to complete projects during the year, and this is one of them: BitRuisseau, which I'm doing for [XCarrel](https://github.com/XCarrel). We have 24 periods of 45 minutes, but I'm putting in a lot more effort than that.

### What?
murcott is a music player inspired by Clementine, written in Rust.
### Who?
People who want to listen to music and download it from other people.
### Functionality?
- Download music
- Play music
- Make playlists

### Why?
When it's time to listen to music, I was never satisfied with Spotify because you cannot use high-quality audio or play your own files. Then I discovered foobar2000 and Clementine, and I love the way they work.

So when we were asked at school to build a music player, I decided to make one that looks like them, with the things I like and without the things I don't.

We also needed to add P2P downloads, which is a great way to let everyone share their music with everyone else.

### Stack?
I'm using Rust because it's reliable and fun to learn.
For this project, it's interesting to use a different language from the others, to see if the compatibility is great.

I'm also building the P2P server. *What is a P2P server?* You need a server to know who is connected to the network and to ask who has which file. The P2P part is downloading files directly between two clients.

### Learning objectives
The main objective is to learn about **distributed systems**: every student in the class is building an app like this one, and all of them must work together.

To get there, I had to learn about egui for the UI/UX, P2P for downloading music between users, UDP for communication, Ed25519 to secure the app, and Protobuf to define message schemas and compile them. I will definitely use all of these in the future.