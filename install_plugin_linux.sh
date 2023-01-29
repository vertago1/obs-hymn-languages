#!/bin/bash -ex

plugin_name="obs_hymn_languages"
plugin_path="${HOME}/.config/obs-studio/plugins/${plugin_name}"
locale_path="${plugin_path}/data/${plugin_name}"
binary_path="${plugin_path}/bin/64bit"
libname="lib${plugin_name}.so"
release="target/release/${libname}"
debug="target/debug/${libname}"

newest="${debug}"
if [[ -f "${release}" ]] && [[ ! -f "${debug}" || "${release}" -nt "${debug}" ]]; then
  newest="${release}"
elif [[ ! -f "${debug}" ]]; then
  echo "Run cargo build first." >&2
  exit 1
fi

rm -rf "${plugin_path}"

mkdir -p "${binary_path}"
cp "${newest}" "${binary_path}/"

mkdir -p "${locale_path}"
cp -rPp "locale" "${locale_path}/"
