# pe-compass
A Study of PE Format through the RUST programming language.

# PROJECT STATUS
** IN DEV-MODE** Do not download or use this for your environment yet.

# Documentation Articles
* PE Rich Data Structure: Undocumented: http://bytepointer.com/articles/the_microsoft_rich_header.htm
* PE Things They Did not tell you...: http://bytepointer.com/articles/rich_header_lifewire_vxmags_29A-8.009.htm
* PE MindMap By Ero Carrera: http://www.openrce.org/reference_library/files/reference/PE%20Format.pdf
* PE MSDN Arcticle:  https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

# To Do
* Implement PE Renderer
* Implement Database Workers
* Optimmization Parsing

# Current Progress
Current Code Base is parsing the following structs, validation in progress.

```rust
/// Inspection Code Now returns an enum "PE_FILE" that holds either of
/// a 32 or 64 Bit pe optional headers object


File Size:          944840
Bytes Content Len:  944840
Vec Content:        944840
```
```json
// Json Output of PE File Object
// Imports Completed
{
  "pename": "sqlite3x86.dll",
  "petype": 267,
  "ImageDosHeader": {
    "e_magic": 23117,
    "e_cblp": 144,
    "e_cp": 3,
    "e_crlc": 0,
    "e_cparhdr": 4,
    "e_minalloc": 0,
    "e_maxalloc": 65535,
    "e_ss": 0,
    "e_sp": 184,
    "e_csum": 0,
    "e_ip": 0,
    "e_cs": 0,
    "e_lfarlc": 64,
    "e_ovno": 0,
    "e_res": [
      0,
      0,
      0,
      0
    ],
    "e_oemid": 0,
    "e_oeminfo": 0,
    "e_res2": [
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0
    ],
    "e_lfanew": 128
  },
  "ImageDosStub": "!This program cannot be run in DOS mode",
  "ImageNtHeaders": {
    "x86": {
      "Signature": 17744,
      "FileHeader": {
        "Machine": 332,
        "NumberOfSections": 18,
        "TimeDateStamp": 1580156954,
        "PointerToSymbolTable": 816128,
        "NumberOfSymbols": 4172,
        "SizeOfOptionalHeader": 224,
        "Characteristics": 8454
      },
      "OptionalHeader": {
        "Magic": 267,
        "MajorLinkerVersion": 2,
        "MinorLinkerVersion": 25,
        "SizeOfCode": 635392,
        "SizeOfInitializedData": 747520,
        "SizeOfUninitializedData": 2560,
        "AddressOfEntryPoint": 5120,
        "BaseOfCode": 4096,
        "BaseOfData": 643072,
        "ImageBase": 1642070016,
        "SectionAlignment": 4096,
        "FileAlignment": 512,
        "MajorOperatingSystemVersion": 4,
        "MinorOperatingSystemVersion": 0,
        "MajorImageVersion": 1,
        "MinorImageVersion": 0,
        "MajorSubsystemVersion": 4,
        "MinorSubsystemVersion": 0,
        "Win32VersionValue": 0,
        "SizeOfImage": 860160,
        "SizeOfHeaders": 1536,
        "CheckSum": 958289,
        "Subsystem": 3,
        "DllCharacteristics": 0,
        "SizeOfStackReserve": 2097152,
        "SizeOfStackCommit": 4096,
        "SizeOfHeapReserve": 1048576,
        "SizeOfHeapCommit": 4096,
        "LoaderFlags": 0,
        "NumberOfRvaAndSizes": 16,
        "DataDirectory": [
          36979669151744,
          13503377924096,
          5119601774592,
          0,
          0,
          58239757295616,
          0,
          0,
          0,
          103079968772,
          0,
          0,
          1872606487024,
          0,
          0,
          0
        ]
      }
    }
  },
  "ImageDataDirectory": {
    "IMAGE_DIRECTORY_ENTRY_TLS": {
      "VirtualAddress": 753668,
      "Size": 24
    },
    "IMAGE_DIRECTORY_ENTRY_BASERELOC": {
      "VirtualAddress": 761856,
      "Size": 13560
    },
    "IMAGE_DIRECTORY_ENTRY_IAT": {
      "VirtualAddress": 745968,
      "Size": 436
    },
    "IMAGE_DIRECTORY_ENTRY_EXPORT": {
      "VirtualAddress": 733184,
      "Size": 8610
    },
    "IMAGE_DIRECTORY_ENTRY_IMPORT": {
      "VirtualAddress": 745472,
      "Size": 3144
    },
    "IMAGE_DIRECTORY_ENTRY_RESOURCE": {
      "VirtualAddress": 757760,
      "Size": 1192
    }
  },
  "ImageSectionHeaders": {
    "/92": {
      "Name": [
        47,
        57,
        50,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 656,
      "VirtualAddress": 856064,
      "SizeOfRawData": 1024,
      "PointerToRawData": 815104,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1108344896
    },
    "/45": {
      "Name": [
        47,
        52,
        53,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 6784,
      "VirtualAddress": 831488,
      "SizeOfRawData": 7168,
      "PointerToRawData": 796672,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1108344896
    },
    "/70": {
      "Name": [
        47,
        55,
        48,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 617,
      "VirtualAddress": 843776,
      "SizeOfRawData": 1024,
      "PointerToRawData": 806400,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1108344896
    },
    ".tls": {
      "Name": [
        46,
        116,
        108,
        115,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 32,
      "VirtualAddress": 753664,
      "SizeOfRawData": 512,
      "PointerToRawData": 733184,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 3224371264
    },
    ".bss": {
      "Name": [
        46,
        98,
        115,
        115,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 2088,
      "VirtualAddress": 729088,
      "SizeOfRawData": 0,
      "PointerToRawData": 0,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 3227517056
    },
    ".rdata": {
      "Name": [
        46,
        114,
        100,
        97,
        116,
        97,
        0,
        0
      ],
      "VirtualSize": 75668,
      "VirtualAddress": 651264,
      "SizeOfRawData": 75776,
      "PointerToRawData": 644608,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1080033344
    },
    "/57": {
      "Name": [
        47,
        53,
        55,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 2236,
      "VirtualAddress": 839680,
      "SizeOfRawData": 2560,
      "PointerToRawData": 803840,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1110442048
    },
    ".text": {
      "Name": [
        46,
        116,
        101,
        120,
        116,
        0,
        0,
        0
      ],
      "VirtualSize": 635012,
      "VirtualAddress": 4096,
      "SizeOfRawData": 635392,
      "PointerToRawData": 1536,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1615855712
    },
    ".idata": {
      "Name": [
        46,
        105,
        100,
        97,
        116,
        97,
        0,
        0
      ],
      "VirtualSize": 3144,
      "VirtualAddress": 745472,
      "SizeOfRawData": 3584,
      "PointerToRawData": 729088,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 3224371264
    },
    ".data": {
      "Name": [
        46,
        100,
        97,
        116,
        97,
        0,
        0,
        0
      ],
      "VirtualSize": 7292,
      "VirtualAddress": 643072,
      "SizeOfRawData": 7680,
      "PointerToRawData": 636928,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 3227516992
    },
    ".rsrc": {
      "Name": [
        46,
        114,
        115,
        114,
        99,
        0,
        0,
        0
      ],
      "VirtualSize": 1192,
      "VirtualAddress": 757760,
      "SizeOfRawData": 1536,
      "PointerToRawData": 733696,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 3224371264
    },
    ".reloc": {
      "Name": [
        46,
        114,
        101,
        108,
        111,
        99,
        0,
        0
      ],
      "VirtualSize": 13560,
      "VirtualAddress": 761856,
      "SizeOfRawData": 13824,
      "PointerToRawData": 735232,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1110442048
    },
    ".edata": {
      "Name": [
        46,
        101,
        100,
        97,
        116,
        97,
        0,
        0
      ],
      "VirtualSize": 8610,
      "VirtualAddress": 733184,
      "SizeOfRawData": 8704,
      "PointerToRawData": 720384,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1076887616
    },
    ".CRT": {
      "Name": [
        46,
        67,
        82,
        84,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 44,
      "VirtualAddress": 749568,
      "SizeOfRawData": 512,
      "PointerToRawData": 732672,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 3224371264
    },
    "/31": {
      "Name": [
        47,
        51,
        49,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 6901,
      "VirtualAddress": 823296,
      "SizeOfRawData": 7168,
      "PointerToRawData": 789504,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1108344896
    },
    "/81": {
      "Name": [
        47,
        56,
        49,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 7379,
      "VirtualAddress": 847872,
      "SizeOfRawData": 7680,
      "PointerToRawData": 807424,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1108344896
    },
    "/19": {
      "Name": [
        47,
        49,
        57,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 39128,
      "VirtualAddress": 782336,
      "SizeOfRawData": 39424,
      "PointerToRawData": 750080,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1108344896
    },
    "/4": {
      "Name": [
        47,
        52,
        0,
        0,
        0,
        0,
        0,
        0
      ],
      "VirtualSize": 728,
      "VirtualAddress": 778240,
      "SizeOfRawData": 1024,
      "PointerToRawData": 749056,
      "PointerToRelocations": 0,
      "PointerToLinenumbers": 0,
      "NumberOfRelocations": 0,
      "NumberOfLinenumbers": 0,
      "Characteristics": 1111490624
    }
  },
  "ImageDLLImports": [
    {
      "name": "kernel32.dll",
      "imports": 79,
      "functions": [
        "AreFileApisANSI",
        "CloseHandle",
        "CreateFileA",
        "CreateFileMappingA",
        "CreateFileMappingW",
        "CreateFileW",
        "CreateMutexW",
        "DeleteCriticalSection",
        "DeleteFileA",
        "DeleteFileW",
        "EnterCriticalSection",
        "FlushFileBuffers",
        "FlushViewOfFile",
        "FormatMessageA",
        "FormatMessageW",
        "FreeLibrary",
        "GetCurrentProcess",
        "GetCurrentProcessId",
        "GetCurrentThreadId",
        "GetDiskFreeSpaceA",
        "GetDiskFreeSpaceW",
        "GetFileAttributesA",
        "GetFileAttributesExW",
        "GetFileAttributesW",
        "GetFileSize",
        "GetFullPathNameA",
        "GetFullPathNameW",
        "GetLastError",
        "GetModuleHandleA",
        "GetProcAddress",
        "GetProcessHeap",
        "GetSystemInfo",
        "GetSystemTime",
        "GetSystemTimeAsFileTime",
        "GetTempPathA",
        "GetTempPathW",
        "GetTickCount",
        "GetVersionExA",
        "GetVersionExW",
        "HeapAlloc",
        "HeapCompact",
        "HeapCreate",
        "HeapDestroy",
        "HeapFree",
        "HeapReAlloc",
        "HeapSize",
        "HeapValidate",
        "InitializeCriticalSection",
        "InterlockedCompareExchange",
        "LeaveCriticalSection",
        "LoadLibraryA",
        "LoadLibraryW",
        "LocalFree",
        "LockFile",
        "LockFileEx",
        "MapViewOfFile",
        "MultiByteToWideChar",
        "OutputDebugStringA",
        "OutputDebugStringW",
        "QueryPerformanceCounter",
        "ReadFile",
        "SetEndOfFile",
        "SetFilePointer",
        "SetUnhandledExceptionFilter",
        "Sleep",
        "SystemTimeToFileTime",
        "TerminateProcess",
        "TlsGetValue",
        "TryEnterCriticalSection",
        "UnhandledExceptionFilter",
        "UnlockFile",
        "UnlockFileEx",
        "UnmapViewOfFile",
        "VirtualProtect",
        "VirtualQuery",
        "WaitForSingleObject",
        "WaitForSingleObjectEx",
        "WideCharToMultiByte",
        "WriteFile"
      ]
    },
    {
      "name": "msvcrt.dll",
      "imports": 28,
      "functions": [
        "__dllonexit",
        "__setusermatherr",
        "_amsg_exit",
        "_beginthreadex",
        "_endthreadex",
        "_errno",
        "_initterm",
        "_iob",
        "_lock",
        "_onexit",
        "_unlock",
        "abort",
        "calloc",
        "fprintf",
        "free",
        "fwrite",
        "localtime",
        "malloc",
        "memcmp",
        "memmove",
        "qsort",
        "realloc",
        "strcmp",
        "strcspn",
        "strlen",
        "strncmp",
        "strrchr",
        "vfprintf"
      ]
    }
  ]
}

```
