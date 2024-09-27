# Ethernet PAUSE frame emitter

To debug home network problems:
<https://lucumr.pocoo.org/2020/7/6/usb-c-network-hubs/>

## Findings

### TP Link TP-SG1008P switch

TP-SG1008P is broken.
Spamming PAUSE frames from a device behind this switch causes the entire network (or at least everything connected through this switch) to go down within about a minute.
Maybe the switch has a single buffer and as it can't send to our device (PAUSE), it also can't send to any other device (head-of-line blocking essentially) and so everything breaks down.

### TP Link TL-SG105 switch

TL-SG105 is broken.
Spamming PAUSE frames from a device behind this switch causes the entire network (including stuff connected upstream from this switch) to go down within about a minute.

### Netgear R6220 router

R6220 (up to date firmware V1.1.0.114_1.0.1) is broken.
Spamming PAUSE frames from a device directly connected to this router's switch causes connectivity on all devices on the home network to fail within about a minute.

### Deco M5

In access point mode, (firmware 1.7.6).
Works without issue.
Spamming PAUSE frames from a device directly connected to the Deco's switch does not produce any problems.
