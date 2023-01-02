# druid-chat-proof-of-concept-
An experiment where we use druid to create a chat window.

The purpose of this repo is to make a proof of concept that demonstrates that fltk is a good enough
tool to create a user interface for a modern instant messaging client.

### Building and running

Running this, once dependencies are resolved, is as simple as running `cargo run`

The most important dependencies are `rust` and `cargo`, but on Linux you may need to also install
some other packages to resolve linking errors.

On Fedora, the packages you need to install are one of:
```
gtk2-devel
webkit2gtk3-devel
```

On ubuntu and debian:
```
libgtk-3-dev
```

On FreeBSD:
```
pkg_add gtk+3
```

On Mac OS, you probably need to install xcode.

### Goals

The goals for this project:
- Not look like this was designed in the 1990s or early 2000s
- On the top, display the title, description, settings, and a way to view users in the chat
- On the bottom, display an editor and send message button
- In the middle, display a timeline that shows messages, who sent the messages, and the timestamps
- Support chat bubbles, as well as IRC or slack timeline styles
- Support message grouping, so messages sent within a minute of each other from the same person, uninterrupted, are grouped to reduce clutter.
- Load more messages while scrolling up, and good management of resources while scrolling far
- Use less RAM than most modern applications
- Have minimal idle CPU usage
- Allow selection of text in the timeline

Nice-to-haves:
- Editor expands once it gets full, up to a point when it reverts to scolling
- Animations of the buttons and addition of new messages on send for feedback to user
- Spell check
- Fractional scaling support
- Easy theming
- Always-on-top support

