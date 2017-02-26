# Wayland protocol structure

Before detailing the protocol contents, we need to have a look at its structure.

This is an asynchronous message-based protocol, with an object-oriented structure.

## Objects, requests and events

Messages are exchanged between the client and the server via a common pool of objects. Each message
is associated to an object and can refer to other objects. Some messages can result in the creation
or the destruction of objects.

Messages from the client to the server are called **requests**, and messages from the server to the
client are called **events**.

> **Examples:**
>
> - A client can have an object that represents the keyboard. This object will receive an event each
>   time the user presses or releases a key.
> - A client can have an object representing a surface on the screen. It can send a request to the
>   server to associate a buffer to this surface, to define its contents in terms of pixel values.

All objects are created by other objects, most of them via a request. The set of all living objects
in a wayland session can thus be organised as a tree, on top of which lives a special object named
`wl_display`.

## The display, the registry and the globals

A client session starts with a single object: the `wl_display`. This object represents the connexion
to the server, an remains alive until the end. This is the entry point of the wayland protocol: from
it instances of `wl_registry` can be created.

This registry is the real heart of the protocol. Upon creation, it will receive a stream of events
that list to the client the set of **globals** the server presents to it. And this registry
then allows the user to instanciate these global objects.

Globals are of two kind:

- Singleton globals, that represent a capability of the compositor
  - *For example, `wl_shm` represents the capability to manipulate shared-memory buffers.*
- Not-Singleton globals[^1], that reprensent a device the compositor has access to
  - *For example, the compositor can advertise a `wl_output` for each screen connected to the
    computer.*

The second kind can appear and dissapear during a session, as devices are plugged or unpugged from
the computer.

In any case, the client can, from the registry, create any number of instance of each global, under
the form of concrete wayland objects.

In the end, this kind of hierachy can be expected:

```text
+------------+      +-------------+      +---------------+      +------------+
| wl_display |----->| wl_registry |--+-->| wl_compositor |--+-->| wl_surface |
+------------+      +-------------+  |   +---------------+  |   +------------+
                                     |                      |
                                     |                      |   +------------+
                                     |                      +-->| wl_surface |
                                     |                          +------------+
                                     |
                                     |   +--------+      +-------------+      +-----------+
                                     +-->| wl_shm |----->| wl_shm_pool |--+-->| wl_buffer |
                                     |   +--------+      +-------------+  |   +-----------+
                                     |                                    |
                                     |                                    |   +-----------+
                                     |                                    +-->| wl_buffer |
                                     |                                        +-----------+
                                     |
                                     |   +---------+      +-------------+
                                     +-->| wl_seat |----->| wl_keyboard |
                                         +---------+      +-------------+
```

*(The details of what these objects are and do will come later, in the
[core protocol](./wayland/p_core.html) section.)*

## Protocol extensions

The set of objects and the list of their requests and events is defined in
[a XML file](https://cgit.freedesktop.org/wayland/wayland/tree/protocol/wayland.xml).
This approach allows for extensions to this protocol to be easily defined.

A **protocol extension** is just an other XML file, which defines an other set of objects, some of
them being globals, and thus serving as the entry points for this protocol extension.

All compositors and clients are expected to fully implement the core protocol. However, nothing is
forced regarding extensions. A client can know that a server supports a given extension if the
global(s) it defines are advertized in the events of the registry. If they are not, then the
compositor does not support it and the client can act accordingly (falling-back to using only the
core protocol, or erroring out if the extension is crutial to the program).

A compositor cannot force the clients to support an extension.

## API versionning

The APIs defined by the protocol files are versionned as follows:

- Each global defines a sub-hierarchy of the objects that it can directly or indirectly create.
  This whole hierarchy shares the same version number.
- Every time a request or an event is added to an object, the whole hierarchy it belongs to has
  its version bumped (but several modifications can be made in a single version bump).
- A compositor advertizes via the registry the maximum version of each global it supports, and
  must support any previous version as well.
- A client, when instanciating a global, can choose any version of it between 1 and the maximum
  supported by the compositor. 

-------
[^1] Sorry for the lack of a better name...