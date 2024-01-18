<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.png" alt="Logo" width="240" height="240">
  </a>

  <h3 align="center">UAM CLI</h3>

  <p align="center">
    Command Line Interface client for the Unity Asset Manager API
    <br />
    <a href="https://github.com/jchultarsky101/uamcli"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jchultarsky101/uamcli">View Demo</a>
    ·
    <a href="https://github.com/jchultarsky101/uamcli/issues">Report Bug</a>
    ·
    <a href="https://github.com/jchultarsky101/uamcli/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](images/screenshot.png) -->

The [Unity Asset Manager](https://unity.com/products/asset-manager) is a great way to manage digital 3D (and other) assets. The Unity team provides a family of helpful APIs for different purposes. One of those is specifically
designed for interfacing with the Asset Manager. Unity also provides useful SDKs to help with the development of client applications.

The reason for this project is to implement a client for those APIs in the Rust language. 

Here's why:
* Rust is quickly becoming very popular with developers
* There is no SDK (as of end of 2023) for Rust
* We like Unity and what they do :smile:

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

This project is built with the wonderful programming language [Rust](https://www.rust-lang.org).

[![Rust][Rust-logo]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Currently the only way to install UAMCLI is to compile it from source code. Clone this repository on your computer, navigate to the project root and compile with Rust:

````nushell
cargo build
````

This will produce an executable for your operating system.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

UAMCLI is a command line utility program. You execute it in the terminal. 

For example, to run it in a BASH shell:

````nushell
uamcli
````
````
╦ ╦╔═╗╔╦╗  ╔═╗╦  ╦
║ ║╠═╣║║║  ║  ║  ║
╚═╝╩ ╩╩ ╩  ╚═╝╩═╝╩



Command Line Interface for the Unity Asset Manager

Usage: uamcli <COMMAND>

Commands:
  config  working with configuration
  asset   Digital asset operations
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print versio
````

The stucture of the command line arguments is inspired by the git command and contains commands and subcommands. 

### Help screen

If no command line arguments are provided, it will display the Usage help as shown before. 


````nushell
uamcli help
````


To get more detailed help on a particular command, enter it after the 'help' command. You can see the available commands.
In the example below, we are showing more details about the usage of the 'config' command.


````nushell
uamcli help config
````
````
working with configuration

Usage: uamcli config <COMMAND>

Commands:
  get     displays configuration
  set     sets configuration property
  export  export the current configuration in a file
  delete  deletes the configuration file
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
````

You can see that the 'config' command has 'export' subcommand. You can take a more detailed look:

````nushell
uamcli help config export
````
````
export the current configuration in a file

Usage: uamcli config export --output <output>

Options:
  -o, --output <output>  output file path
  -h, --help             Print help
  -V, --version          Print version
````

### Configuration

You need to obtain your own Unity account, create an organization and a project. How you do that is beyond the scope of this tool, but you can read their [Getting Started](https://services.docs.unity.com/docs/) documentation.

There are five bits of information that you need to use UAMCLI:

* Organization ID - A numeric identifier for your Unity organization
* Project ID - A alpha-numeric identifier for your Unity project
* Environment ID - A string identifier for your Unity environment (usually 'production')
* Key ID - This is the Key ID for your Unity service account. In UAMCLI it is known as the 'client ID'
* Key Secret - This is the Key Secret for your Unity service account. In UAMCLI it is known as the 'client secret'

The *config set client* command stores the configuration on your system and allows you to make repeated calls later. 

````nushell
uamcli help config set client
````
````
Sets the clinet properties

Usage: uamcli config set client --organization <organization> --project <project> --environment <environment> --client_id <client_id> --client_secret <client_secret>

Options:
  -o, --organization <organization>    organization ID
  -p, --project <project>              tenant ID
      --environment <environment>      Unity environment ID
      --client_id <client_id>          Client ID for authentication
      --client_secret <client_secret>  Client secret for authentication
  -h, --help                           Print help
  -V, --version                        Print version

````

Obviously, you will need to use your own values.

All, except the *client secret* is stored in a YAML config file on your computer. The location of this file depents on the operating system you are using. It is the recommended location for configuration files according to your OS.
If the file does not exist, a new one will be created. You do not need to edit this file manually. We recommend using the UAMCLI commands to manage it.

You can see where the file is stored by using the *config get path* command:

````nushell
uamcli config get path
````

The *client secret* is stored in your operating system's secure credentials manager. It is the vault used by the OS to store sensitive data on your computer. When using UAMCLI you will be prompted by the OS to allow it to store data
there.
In the case of Windows, this would be the Credentials Manager. In the case of MacOS, this would be the Keystore. There are similar facilities for all other operating systems.

To view your current client configuration:

````nushell
uamcli config get client
````

This will output the current configuration as JSON. It will ***NOT*** display your *client secret*. That cannot be retrieved once set. You will have to reset it if you change it.

### Uploading data

The Unity Asset Manager has the concept of an *asset*. An asset is a container that may include one or more files under a common name.
Those files could be anything, but most likelly those would be 3D models. For example, an STL file. To upload data, we use the *asset* command 
with it's *create* subcommand.


````nushell
uamcli asset create --name test1 --data data/sample/test.stl
````
````
{"id":"65a7d8646e7591cfd372ee51","version":"1"}
````

The arguments we provided are as follows:

* --name - this is the desired name of the new asset as it would appear in the Unity's Asset Manager
* --data - the local path of the file we want to upload

If you have more than one file, you can specify the --data argument multiple times as necessary:


````nushell
uamcli asset create --name test1 --data data/sample/test.stl --data data/sample/test2.stl
````
````
{"id":"65a7d8646e7591cfd372ee51","version":"1"}
````

The output of the commands is usually JSON. The UAMCLI is designed to be used together with other tools and perhaps your own custom scripts. The output from UAMCLI is meant to be
used as the input to another program.


### Reading asset data

In the example above, the *id* is the asset ID as recorded in UAM. You can use that ID and the version number to read back the asset data.

````nushell
uamcli asset get --asset-id 65a7d8646e7591cfd372ee51 --asset-version 1
````
````
{"identity":{"id":"65a7d8646e7591cfd372ee51","version":"1"},"name":"test1","description":null,"tags":null,"system_tags":null,"labels":[],"primary_type":"3D Model","status":"Draft","frozen":false,"source_project_id":"dd572c59-893e-4de9-996f-04
a60820083c","project_ids":["dd572c59-893e-4de9-996f-04a60820083c"],"preview_file":"","preview_file_dataset_id":"","datasets":[{"datasetId":"75a02d61-4e83-41a2-b809-86c41453f8b8","name":"Source"},{"datasetId":"608ae6a4-b652-4cd1-9a63-f2bddf4e5
cfd","name":"Preview"}],"metadata":null}
````

We could combine that with the ***jq*** tool to get a better formatted JSON:

````nushell
uamcli asset get --asset-id 65a7d8646e7591cfd372ee51 --asset-version 1 | jq
````
````
{
  "identity": {
    "id": "65a7d8646e7591cfd372ee51",
    "version": "1"
  },
  "name": "test1",
  "description": null,
  "tags": null,
  "system_tags": null,
  "labels": [],
  "primary_type": "3D Model",
  "status": "Draft",
  "frozen": false,
  "source_project_id": "dd572c59-893e-4de9-996f-04a60820083c",
  "project_ids": [
    "dd572c59-893e-4de9-996f-04a60820083c"
  ],
  "preview_file": "",
  "preview_file_dataset_id": "",
  "datasets": [
    {
      "datasetId": "75a02d61-4e83-41a2-b809-86c41453f8b8",
      "name": "Source"
    },
    {
      "datasetId": "608ae6a4-b652-4cd1-9a63-f2bddf4e5cfd",
      "name": "Preview"
    }
  ],
  "metadata": null
}
````

### Listing the assets

To list all available assets in our Unity project, we can use the ***asset search*** command:

````nushell
uamcli asset search
````
````
[{"identity":{"id":"65a7d8646e7591cfd372ee51","version":"1"},"name":"test1","description":null,"tags":[],"system_tags":[],"labels":[],"primary_type":"3D Model","status":"Draft","frozen":false,"source_project_id":"dd572c59-893e-4de9-996f-04a60
820083c","project_ids":["dd572c59-893e-4de9-996f-04a60820083c"],"preview_file":null,"preview_file_dataset_id":"","datasets":null,"metadata":null}]
````

UAMCLI works very well in combination with [NuShell](https://www.nushell.sh). Here is an example of the two working together:

````nushell
uamcli asset search | from json | select identity.id identity.version name
````
````
╭───┬──────────────────────────┬──────────────────┬───────╮
│ # │       identity_id        │ identity_version │ name  │
├───┼──────────────────────────┼──────────────────┼───────┤
│ 0 │ 65a7d8646e7591cfd372ee51 │ 1                │ test1 │
╰───┴──────────────────────────┴──────────────────┴───────╯
````

In this case, we used UAMCLI to fetch the list of available assets and piped the output to NuShell to select only the fields that we are interested. With NuShell you can do further data manupulations, store the results to a file and execute other programs as needed.
It is a great tool to build custom scripts.

### Uploading metadata

An asset contains files, but also can have metadata, which is a collection of key/value pairs that are used to describe the contents. For example, you may have a property named "Material" that indicates the type of material used
to make a 3D object.

While the Unity Asset Manager allows for metadata properties to be declared of different types, the current version UAMCLI only supports text fields. This was done to satisfy a specific requirement of a downstream process. Future versions
may add support for other types (e.g. boolean, etc.).

To upload metadata to an existing asset, you can use the *asset metadata upload* command.

````nushell
uamcli help asset metadata upload
````
````
Usage: uamcli asset metadata upload --asset-id <asset-id> --asset-version <asset-version> --data <data>

Options:
      --asset-id <asset-id>            asset ID
      --asset-version <asset-version>  asset version
      --data <data>                    file containing the metadata in CSV format with two columns: NAME, VALUE
  -h, --help                           Print help
  -V, --version                        Print version
````

It takes the following arguments:
* --asset-id - the ID of an exising asset. See above on how to create a new one.
* --asset-version - the version of the asset as per UAM.
* --data - local path to a file containing the metadata

The current version of UAMCLI uses CSV format for the metadata. It has only two columns: Name, Value. It needs to have a header line with the column names. Here is an example:

````bash
cat data/metadata/metadata.csv
````
````
Name,Value
Material,TPU
License,Apache
Vendor,Non
````

In this example we have a CSV file named *metadata.csv* with a header line with the column names and 3 records. In the first record we specify a property with key 'Material' and value of 'TPU'.

To excute the upload:

````bash
uamcli asset metadata upload --asset-id 65a7d8646e7591cfd372ee51 --asset-version 1 --data data/metadata/metadata.csv
````

If successful, there is no output. Once the command completes, the asset will contain the three metadata properties with their respective values.

***👉 NOTE:***
At the time of writing the Unity Asset Manager is still in beta. Only previously registered metadata field definitions can be used. Make sure you add those definitions to your Unity organization before
attempting to assign values to them. 
In the future we should be able to automatically register new metadata field definitions as needed.

To see the effect of the above command, we can use the *asset get* command again and with the help of NuShell we can subselect the metadata from the output:

````nushell
uamcli asset get --asset-id 65a7d8646e7591cfd372ee51 --asset-version 1 | from json | get metadata
````
````
╭──────────┬────────╮
│ Material │ TPU    │
│ Vendor   │ None   │
│ License  │ Apache │
╰──────────┴────────╯
````

You could save this into the same CSV file format with the header using NuShell:

````nushell
uamcli asset get --asset-id 65a7d8646e7591cfd372ee51 --asset-version 1 | from json | get metadata | columns | each {|c| [['Name', 'Value']; [$c, ($props | get $c)]]} | flatten | to csv | save data/metadata/metadata2.csv
````

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

_The project is work in progress. No release has been provided as of yet. Most of the work is under the 'develop' branch._

- [x] Add Changelog
- [x] Add login/logoff functions
- [x] Implement basic asset operations (create, search, metadata)
- [x] Implement basic file operations (upload)
- [ ] Test suite
- [ ] CI/CD
- [ ] Support for multiple organizations and projects
- [ ] Create documentation via Github Pages

See the [open issues](https://github.com/jchultarsky101/uamcli/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@jchultarsky101](https://twitter.com/jchultarsky101) - jchultarsky@physna.com

Project Link: [https://github.com/jchultarsky101/uamcli](https://github.com/jchultarsky101/uamcli)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Use this space to list resources you find helpful and would like to give credit to. I've included a few of my favorites to kick things off!

* [Choose an Open Source License](https://choosealicense.com)
* [GitHub Emoji Cheat Sheet](https://www.webpagefx.com/tools/emoji-cheat-sheet)
* [Img Shields](https://shields.io)
* [GitHub Pages](https://pages.github.com)
* [Font Awesome](https://fontawesome.com)
* [Best-README-Template](https://github.com/othneildrew/Best-README-Template)
* [Clap](https://crates.io/crates/clap)
* [Configuration](https://crates.io/crates/configuration)
* [Dirs](https://crates.io/crates/dirs)
* [Env](https://crates.io/crates/env)
* [Keyring](https://crates.io/crates/keyring)
* [Log](https://crates.io/crates/log)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/jchultarsky101/uamcli.svg?style=for-the-badge
[contributors-url]: https://github.com/jchultarsky101/uamcli/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jchultarsky101/uamcli.svg?style=for-the-badge
[forks-url]: https://github.com/jchultarsky101/uacli/network/members
[stars-shield]: https://img.shields.io/github/stars/jchultarsky101/uamcli.svg?style=for-the-badge
[stars-url]: https://github.com/jchultarsky101/uamcli/stargazers
[issues-shield]: https://img.shields.io/github/issues/jchultarsky101/uamcli.svg?style=for-the-badge
[issues-url]: https://github.com/jchultarsky101/uamcli/issues
[license-shield]: https://img.shields.io/github/license/jchultarsky101/uamcli.svg?style=for-the-badge
[license-url]: https://github.com/jchultarsky101/uamcli/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/julianchultarsky
[product-screenshot]: images/screenshot.png
[Rust-url]: https://www.rust-lang.org/
[Rust-logo]: http://rust-lang.org/logos/rust-logo-blk.svg
