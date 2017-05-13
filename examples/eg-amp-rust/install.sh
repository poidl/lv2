LV2DIR=~/.lv2/
BACKUPDIR=~/programs/backup_lv2/
BUNDLE=eg-amp-rust.lv2
SONAME=libeg_amp_rust.so
DEST=$LV2DIR$BUNDLE
if [ -d "$DEST" ]; then
    if [ -d "$BACKUPDIR/$BUNDLE" ]; then
        rm -r $BACKUPDIR/$BUNDLE
    fi
    mv $DEST $BACKUPDIR/$BUNDLE
fi
cp -r $BUNDLE $LV2DIR
cp ../../target/debug/examples/$SONAME $DEST
