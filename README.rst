gst-log-diff
============

Usage:

::

  gst-log-diff 0.1.0
  Philippe Normand <philn@igalia.com>
  Print the difference between two GStreamer log files

  USAGE:
    gst-log-diff [OPTIONS] <input1> <input2>

  FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

  OPTIONS:
    -c, --category <category>    Filter on given category. A regular expression can be provided

  ARGS:
    <input1>    Input file1
    <input2>    Input file2

Example output:

::

  $ cargo run --release -- -c "webkit" ~/WebKit/gst-good.log ~/WebKit/gst-bad.log
  00:00:00.376249666 => -Message state-changed received from element wavparse0
  00:00:00.351766965 => +Source element set-up for source
  00:00:00.426176007 => -Cached duration: {-1/1 = NaN}
  00:00:00.429461918 => +Cached duration: {0/1 = Infinity}
  00:00:00.427558570 => -Message state-changed received from element aconv
  00:00:00.429752700 => +Cached duration: {0/1 = Infinity}
  00:00:00.427590129 => -Message state-changed received from element aqueue
  00:00:00.429770139 => +Cached duration: {0/1 = Infinity}
  00:00:00.427616005 => -Message state-changed received from element scaletempo0
  00:00:00.429915889 => +Cached duration: {0/1 = Infinity}
  00:00:00.427641245 => -Message state-changed received from element filter-convert
  00:00:00.430046067 => +Cached duration: {0/1 = Infinity}
  00:00:00.427666181 => -Message state-changed received from element abin
  00:00:00.430063950 => +Cached duration: {0/1 = Infinity}
