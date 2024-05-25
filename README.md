## logs
### 24.05.25
```
I attempted to listen to the signal from the Analogue Pocket, but it is not recognized as a device, and the system does not see it. There is a slight progress; the system recognizes it as a device when it is in data transfer mode to the memory card, but that’s not exactly what we need. We need to capture the very first bytes it sends us. Either the AP or the Dock Station should be sending these packets. Once we learn to capture them, we can move forward.

The next step, I think, is to try to listen to the USB port where I connect the device using WireShark. To do this, it is necessary to disable System Integrity Protection on the Mac. Once this is done and it is clearly visible that there is data transmission from the AP or Dock on the port, we can proceed and repeat this procedure using Rust. The next step will likely involve transmitting these bytes between devices. It's possible that the Dock Station only receives and the AP transmits. I hope we can get to this point.
```