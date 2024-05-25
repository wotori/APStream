# OpenAPS

OpenAPS (Open Analogue Pocket Streamer) is a project that aims to stream your gameplay from the Analogue Pocket console to your PC. The goal of this project is to enable a range of functionalities to enhance your gaming experience, including:

- **Gameplay Recording**: Capture your gameplay sessions directly to your PC for later review or sharing.
- **Live Streaming**: Broadcast your gameplay in real-time to platforms like Twitch and YouTube, leveraging the powerful streaming capabilities of OBS (Open Broadcaster Software).

This project was born during this [Reddit discussion](https://www.reddit.com/r/AnaloguePocket/comments/1cwzqur/screen_broadcasting_from_analogue_pocket_to_pc/) where redditors shared their visions and ideas.

Whether you're looking to record your gameplay for personal archives or share your gaming skills with a broader audience, OpenAPS will someday offer the tools you need to elevate your gaming content.


## logs
### 24.05.25

I attempted to listen to the signal from the Analogue Pocket, but it is not recognized as a device, and the system does not see it. There is a slight progress; the system recognizes it as a device when it is in data transfer mode to the memory card, but that’s not exactly what we need. We need to capture the very first bytes it sends us. Either the AP or the Dock Station should be sending these packets. Once we learn to capture them, we can move forward.

The next step, I think, is to try to listen to the USB port where I connect the device using WireShark. To do this, it is necessary to disable System Integrity Protection on the Mac. Once this is done and it is clearly visible that there is data transmission from the AP or Dock on the port, we can proceed and repeat this procedure using Rust. The next step will likely involve transmitting these bytes between devices. It's possible that the Dock Station only receives and the AP transmits. I hope we can get to this point.
