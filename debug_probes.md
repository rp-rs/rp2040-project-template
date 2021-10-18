# Compatible CMSIS-DAP debug probes

## Raspberry Pi Pico

  You can use a second Pico as your debugger.

  -  Download this file: https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2
  - Boot the Pico in bootloader mode by holding the bootset button while plugging it in
  - Open the drive RPI-RP2 when prompted
  - Copy raspberry_pi_pico-DapperMime.uf2 from Downloads into RPI-RP2
  - Connect the debug pins of your CMSIS-DAP Pico to the target one
      - Connect GP2 on the Probe to SWCLK on the Target
      - Connect GP3 on the Probe to SWDIO on the Target
      - Connect a ground line from the CMSIS-DAP Probe to the Target too

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
