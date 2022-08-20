# OvenMitts

A set of tools built around [OvenMediaEngine](https://airensoft.gitbook.io/ovenmediaengine/), which makes hosting your own low-latency streaming server easy.

## Why?

Imagine this: you are playing a game or doing some other work that requires streaming video with very little delay. A lot of existing solutions are subpar in some regards, like Twitch or YouTube, which introduce quite a bit of delay, and are more focused on broadcasting publicly, as opposed to a small group. Discord, while much better suited, requires a subscription for above 720p/30 FPS streaming, and has a bunch of issues on Linux.

Using your own hardware, you can deploy OvenMitts and OvenMediaEngine, which allows you to stream sub-second latency video, without the strict limitations on resolution, frame- and bitrate.

## How?

To be clear: most of the heavy lifting is done by OvenMediaEngine and [OvenPlayer](https://www.ovenmediaengine.com/ovenplayer). However, there are a bunch of issues that make deploying it more cumbersome than it needs to be:

- OME has no concept of users. By default, everyone can stream using a stream key of their liking

- The final streaming URL contains the stream key, effectively leaking it to every viewer

- Besides the OvenPlayer, no actual interface for watching the stream is provided

OvenMitts solves the first two issues by implementing a user system, where every user has a unique stream key. It makes use of the [admission webhooks](https://airensoft.gitbook.io/ovenmediaengine/access-control/admission-webhooks) system to deny non-registered users from streaming and rewrites the URLs of registered users to not contain the stream key. It also provides an interface to watch the streams.

# Current progress

Most of the features listed above are not yet implemented. The admission webhooks are functional, but there is no built-in way to manage users, you'd have to edit the `mitts.sqlite` database manually.

The following features are planned:

- User management
  
  - Create, delete and modify users
  
  - Simple permission system

- Interface
  
  - Watch streams
  
  - Manage user settings
  
  - Support for republishing streams
  
  - Support for private streams
