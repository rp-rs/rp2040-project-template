# Compatible CMSIS-DAP debug probes

## Raspberry Pi Pico

You can use a second Pico as your debugger.

Download the latest official raspberrypi probe firmware supporting CMSIS-DAP: <https://github.com/raspberrypi/debugprobe/releases>

If you are using a Raspberry Pi Pico 1, then grab `debugprobe_on_pico.uf2`. If you're using a Raspberry Pi Pico 2, then grab `debugprobe_on_pico2.uf2`

Then:

1. Put the Pico into USB Mass Storage Mode by holding the BOOTSEL button while connecting it to your computer with a USB cable
2. Open the drive RPI-RP2 when prompted
3. Copy the uf2 firmware file from Downloads into RPI-RP2
4. Connect the debug pins of your CMSIS-DAP Pico to the target one
   - Connect GP2 on the Probe to SWCLK on the Target
   - Connect GP3 on the Probe to SWDIO on the Target
   - Connect a ground line from the CMSIS-DAP Probe to the Target too
5. Finally, ensure both of your boards (debug board and main board) are plugged in to power. You can return to the main quickstart guide.

## WeAct MiniF4

https://therealprof.github.io/blog/usb-c-pill-part1/

## HS-Probe

https://github.com/probe-rs/hs-probe

## ST-LINK v2 clone

It's getting harder to source these with stm32f103's as time goes on, so you might be better off choosing a stm32f103 dev board

Firmware: https://github.com/devanlai/dap42

## LPC-Link2

https://www.nxp.com/design/microcontrollers-developer-resources/lpc-link2:OM13054

## MCU-Link

https://www.nxp.com/part/MCU-LINK#/

## DAPLink

You can use DAPLink firmware with any of it's supported chips (LPC4322, LPC11U35, K20, K22, KL26). You'll need to use the 'develop' branch to use GCC to build it. You'll need to find a chip with the correct

Firmware source: https://github.com/ARMmbed/DAPLink/tree/develop
