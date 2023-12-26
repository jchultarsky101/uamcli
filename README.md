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

[![Product Name Screen Shot][product-screenshot]](images/screenshot.png)

The [Unity Asset Manager](https://unity.com/products/asset-manager) is a great way to manage digital 3D (and other) assets. The Unity team provides a family of helpful APIs for different purposes. One of those is specificlly
designed for interfacing with the Asset Manager. Unity also provides useful SDKs to help with the development of client applications.

The reason for this project is to implement a client for those APIs in the Rust language. 

Here's why:
* Rust is quickly becoming very popular with developers
* There is no SDK (as of end of 2023) for Rust
* We like Unity and what they do :smile:

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

This project is built with the wonderful programming language Rust.

[![Rust][Rust-logo]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

We provide pre-compiled binaries for MacOS and Windows. If you need to execute it on another platform, you can compile from source, or open an issue and we will be happy to provide.

The application runs in the terminal. Therefore, you must have a terminal client and know how to use it.

### Installation

_The following are instructions on how to install the pre-compiled binary_

1. Download the binary for your platform from the link listed under the latest release
2. Move the binary file to a desired directory on your computer
3. (Optional) Add the binary to your PATH or create an alias for it to make it easy to execute from any directory

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

UAMCLI is a command line utility program. You execute it in the terminal. 

For example, to run it in a BASH shell:

````bash
uamcli
Command Line Interface for the Unity Asset Manager

Usage: uamcli <COMMAND>

Commands:
  config  working with configuration
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
````

If no command line arguments are provided, it will display the Usage help. You can see the available commands. The same text will be displayed if you provide the command 'help' as argument:

````bash
uamcli help
````

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

_The project is work in progress. No release has been provided as of yet. Most of the work is under the 'develop' branch._

- [x] Add Changelog
- [ ] Add login/logoff functions
- [ ] Implement project operations
- [ ] Implement file operations
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

Your Name - [@your_twitter](https://twitter.com/your_username) - email@example.com

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
