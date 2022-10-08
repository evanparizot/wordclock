# WordClock

This project is not a word clock based on the [wikipedia](https://en.wikipedia.org/wiki/Word_clock) definition, but rather just an aesthetic choice of representing time. Rather than showing something like a clock face or time in 12 hour / 24 hour format, a wordclock spells out the time using words. One of the more well known examples of wordclocks being produced is from the company [QLOCKTWO](https://qlocktwo.com/us?___from_store=de). Unfortunately, at it's retail price of $1,500+, they can be pretty expensive.

This project is to provide files for those who wish to build their own instead. It's been inspired by repositories such as [this](https://github.com/LikeClockwork/LargeWordClock) on providing both the pcb schematics, as well source code for anyone to look at. 

## Project Overview

Included in this repository are the graphics used to make up the clock face, the pcb schematics that outline the circuitry for the clock, as well the source code for the firmware used to run the wordclock. You can find each in their respective repostories. 

The pcb schematics were made using a mixture of Altium Designer and KiCad. The microcontroller is a variant of the STM32, with source code written in Rust. Each sub folder has further details on how to work with each of those components.

## Development Progress

Developmet of this project is ongoing and will be for some time. I will be keeping milestones in a Github project for the repository to track progress.