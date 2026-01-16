# NT kernel support

There are two main systems that will be focused, WindowsOS and ReactOS.

## Dependencies

This module will avoid depending on general windows DLLs, drivers and libs to improve support for ReactOS. For now, the only dependency is ntdll.dll (not optional for windows app development). Yes, you read right, not even system32.dll is used.

## Useful links

https://learn.microsoft.com/en-us/windows/win32/api/_dwm/

https://doxygen.reactos.org/d3/d44/dwmapi_8h_source.html