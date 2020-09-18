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
	usage = "usage: verifyimg -t (jpg|png|gif) IMAGE_FILE"
)

func main() {
	var (
		imageType string
	)

	flag.StringVar(&imageType, "t", "", "image type should be either of jpg, png, gif")
	flag.Parse()
	if imageType == "" {
		fmt.Println(usage)
		os.Exit(1)
	}

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
		fmt.Printf("can't read file: %s\n", file)
		os.Exit(1)
	}
	defer func() { _ = f.Close() }()

	imageType = strings.ToLower(imageType)
	switch imageType {
	case "jpg", "jpeg":
		_, err = jpeg.Decode(f)
	case "png":
		_, err = png.Decode(f)
	case "gif":
		_, err = gif.Decode(f)
	default:
		fmt.Printf("invalid image type %s; either of jpg, png, gif should be specified by -t option\n", imageType)
		os.Exit(1)
	}

	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	fmt.Printf("%s is valid %s image\n", file, imageType)
}
