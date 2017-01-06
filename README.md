Raugtool
========

This project was created in order to provide support for relocating tree entries in augeas. This is an *alpha* version.

Augtool, and the augeas API, does not provide a way to move a node to a specified place, to ensure that nodes are ordered properly.

Raugtool fixes that by providing an additional command on top of one can do in augtool:

    reorder SRC before|after DEST

It works by moving each node matched by SRC before the first node matched by DEST, or after the last one.

Since augeas' API does not allow one to do that directly, this is what is done for each node (in pseudo-augtool-code):

    $label = label $src
    insert augtool-marker-node before $dest[1]
    mv $src $dest[1]/../augtool-marker-node
    rename $dest[1]/../augtool-marker-node $label
