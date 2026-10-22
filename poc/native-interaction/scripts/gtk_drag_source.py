#!/usr/bin/python3.12
"""Fonte de arraste X11/GTK3 para o smoke test Linux: oferece text/uri-list.
Uso: gtk_drag_source.py ARQ1 [ARQ2 ...]  (caminhos absolutos)"""
import sys, pathlib
import gi
gi.require_version("Gtk", "3.0")
from gi.repository import Gtk, Gdk

uris = [pathlib.Path(p).resolve().as_uri() for p in sys.argv[1:]]
win = Gtk.Window(title="hf-drag-source")
win.set_default_size(200, 100)
win.move(100, 100)
box = Gtk.EventBox()
box.add(Gtk.Label(label="arraste daqui"))
win.add(box)
target = Gtk.TargetEntry.new("text/uri-list", 0, 0)
box.drag_source_set(Gdk.ModifierType.BUTTON1_MASK, [target], Gdk.DragAction.COPY)
box.connect("drag-data-get", lambda w, ctx, sel, info, t: sel.set_uris(uris))
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()
