package main

import (
	"flag"
	"fmt"
	"image/gif"
	"image/jpeg"
	"image/png"
	"os"
	"strings"
)

const (
	usage = "Usage: verifyimg -t (jpeg|png|gif) <file>"
)

func main() {
	var (
		imageType string
	)

	flag.StringVar(&imageType, "t", "", "image type")
	flag.Parse()

	file := flag.Arg(0)
	if file == "" {
		fmt.Println(usage)
		os.Exit(1)
	}

	var (
		err error
		f *os.File
	)

	f, err = os.Open(file)
	if err != nil {
		fmt.Printf("can't open %s\n", file)
		os.Exit(1)
	}
	defer func() { _ = f.Close() }()

	imageType = strings.ToLower(imageType)
	switch imageType {
	case "jpeg", "jpg":
		_, err = jpeg.Decode(f)
	case "png":
		_, err = png.Decode(f)
	case "gif":
		_, err = gif.Decode(f)
	case "":
		fmt.Println(usage)
		os.Exit(1)
	default:
		fmt.Printf("invalid image type %s specified; either of jpeg, png, gif should be specified by -t option\n", imageType)
		os.Exit(1)
	}

	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	fmt.Println("ok")
}
