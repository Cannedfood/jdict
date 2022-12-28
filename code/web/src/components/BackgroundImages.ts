export interface LodLevel {
    width: number,
    height: number,
    url: string
}

export interface BackgroundImage {
    credit: string,
    levels: LodLevel[],
    style: 'light' | 'dark',
}

const Sizes: Array<[number, number, string, number]> = [
    [640,  360,   '-360p.webp', 80],
    [1280, 720,   '-720p.webp', 90],
    [1920, 1080, '-1080p.webp', 90],
    [2560, 1440, '-1440p.webp', 90],
    [3840, 2160, '-2160p.webp', 90],
    [ 360,  640,  '-360p-vertical.webp', 80],
    [ 720, 1280,  '-720p-vertical.webp', 90],
    [1080, 1920, '-1080p-vertical.webp', 90],
    [1440, 2560, '-1440p-vertical.webp', 90],
    [2160, 3840, '-2160p-vertical.webp', 90],
]

function generated_files(s: string): LodLevel[] {
    return Sizes.map(
        ([w, h, postfix, quality]) => ({
            width: w,
            height: h,
            url: s.replace('.jpg', postfix)
        })
    )
}

export
const BackgroundImages: Record<string, BackgroundImage> = {
    'D68ADLeMh5Q': {
        credit: `Photo by <a href="https://unsplash.com/@dnevozhai?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Denys Nevozhai</a> on <a href="https://unsplash.com/?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>`,
        levels: generated_files("images/denys-nevozhai-D68ADLeMh5Q-unsplash.jpg"),
        style: 'dark',
    },
    'oCZHIa1D4EU': {
        style: 'dark',
        credit: `Photo by <a href="https://unsplash.com/@jasebloor?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Jase Bloor</a> on <a href="https://unsplash.com/?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>`,
        levels: generated_files("images/jase-bloor-oCZHIa1D4EU-unsplash.jpg")
    },
    'fqF0ZkRTs5E': {
        credit: `Photo by <a href="https://unsplash.com/@alex_rainer?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">𝗔𝗹𝗲𝘅 𝘙𝘢𝘪𝘯𝘦𝘳</a> on <a href="https://unsplash.com/?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>`,
        levels: generated_files("images/alex_rainer-fqF0ZkRTs5E-unsplash.jpg"),
        style: 'dark',
    },
    'McsNra2VRQQ': {
        credit: `Photo by <a href="https://unsplash.com/@ajny?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">AJ</a> on <a href="https://unsplash.com/?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>`,
        levels: generated_files("images/aj-McsNra2VRQQ-unsplash.jpg"),
        style: 'light',
    },
    '5bXUxtRXi8w': {
        credit: `Photo by <a href="https://unsplash.com/@tata186?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Tatiana Rodriguez</a> on <a href="https://unsplash.com/?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>`,
        levels: generated_files("images/tatiana-rodriguez-5bXUxtRXi8w-unsplash.jpg"),
        style: 'light',
    },
    'x95-rxpCmkE': {
        credit: `Photo by <a href="https://unsplash.com/@alexandrelallemand?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">ALEXANDRE LALLEMAND</a> on <a href="https://unsplash.com/?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>`,
        levels: generated_files("images/alexandre-lallemand-x95-rxpCmkE-unsplash.jpg"),
        style: 'dark',
    }
}