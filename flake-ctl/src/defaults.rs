//
// Copyright (c) 2022 Elektrobit Automotive GmbH
// Copyright (c) 2023 Marcus Schäfer
//
// This file is part of flake-pilot
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const PODMAN_PILOT: &str =
    "/usr/bin/podman-pilot";
pub const FIRECRACKER_PILOT: &str =
    "/usr/bin/firecracker-pilot";
pub const PODMAN_PATH:&str =
    "/usr/bin/podman";
pub const FLAKE_TEMPLATE_CONTAINER:&str =
    "/etc/flakes/container-flake.yaml";
pub const FLAKE_TEMPLATE_FIRECRACKER:&str =
    "/etc/flakes/firecracker-flake.yaml";
pub const FIRECRACKER_REGISTRY_DIR:&str =
    "/var/lib/firecracker";
pub const FIRECRACKER_IMAGES_DIR:&str =
    "/var/lib/firecracker/images";
pub const FIRECRACKER_INITRD_NAME:&str =
    "initrd";
pub const FIRECRACKER_KERNEL_NAME:&str =
    "kernel";
pub const FIRECRACKER_ROOTFS_NAME:&str =
    "rootfs";
pub const FIRECRACKER_SCI:&str =
    "/usr/lib/flake-pilot/sci";
pub const FLAKES_STORAGE:&str =
    "/etc/flakes/storage.conf";
pub const FLAKES_REGISTRY_RUNROOT: &str =
    "/run/flakes";
