

To setup the power button monitor to always start on reboot:

- Move the `.service` file in this directory to `/lib/systemd/system/rustypowerbutt.service`
- Run `sudo systemctl enable rustypowerbutt.service`
- Reboot.  When you restart you should see the power button process in `ps ax |grep rusty`
