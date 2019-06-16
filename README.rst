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
  -Message state-changed received from element wavparse0
  +Source element set-up for source
  -Cached duration: {-1/1 = NaN}
  +Cached duration: {0/1 = Infinity}
  -Message state-changed received from element aconv
  +Cached duration: {0/1 = Infinity}
  -Message state-changed received from element aqueue
  +Cached duration: {0/1 = Infinity}
  -Message state-changed received from element scaletempo0
  +Cached duration: {0/1 = Infinity}
  -Message state-changed received from element filter-convert
  +Cached duration: {0/1 = Infinity}
  -Message state-changed received from element abin
  +Cached duration: {0/1 = Infinity}
