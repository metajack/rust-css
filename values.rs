/*!
The representation of CSS property values

Enums are named after the property. Variants have the same name + the
name of the value used in the spec. This leads to some verbose names,
e.g.:

The property 'background-color' and the specified value called '<color>'
in the spec lead to the variant CSSBackgroundColorColor(Color).

At least it's consistent though.
*/

use core::cmp::Eq;
use std::net::url::Url;
use netsurfcss::stylesheet::CssStylesheet;
use units::{Length, AbsoluteSize, RelativeSize,
            BoxSizing, BoxLength, BoxPercent, BoxAuto, Px, Em, Pt};
use units::GenericFontFamily;
use color::Color;
use std::cmp::FuzzyEq;

/** A partial CSS value, before inheritance has been resolved */
#[deriving_eq]
pub enum CSSValue<T> {
    Inherit,
    Specified(T),
}


// CSS 2.1, Section 8 - Box model

#[deriving_eq]
pub enum CSSMargin {
    CSSMarginLength(Length),
    CSSMarginPercentage(float),
    CSSMarginAuto
}

pub enum CSSPadding {
    CSSPaddingLength(Length),
    CSSPaddingPercentage(float)
}

#[deriving_eq]
pub enum CSSBorderWidth {
    CSSBorderWidthThin,
    CSSBorderWidthMedium,
    CSSBorderWidthThick,
    CSSBorderWidthLength(Length)
}

pub enum CSSBorderColor {
    CSSBorderColorColor(Color),
    CSSBorderColorTransparent
}

pub enum CSSBorderStyle {
    CSSBorderStyleNone,
    CSSBorderStyleHidden,
    CSSBorderStyleDotted,
    CSSBorderStyleDashed,
    CSSBorderStyleSolid,
    CSSBorderStyleDouble,
    CSSBorderStyleGroove,
    CSSBorderStyleRidge,
    CSSBorderStyleInset,
    CSSBorderStyleOutset,
}

// CSS 2.1, Section 9 - Visual formatting model

#[deriving_eq]
pub enum CSSDisplay {
    CSSDisplayInline,
    CSSDisplayBlock,
    CSSDisplayListItem,
    CSSDisplayInlineBlock,
    CSSDisplayTable,
    CSSDisplayInlineTable,
    CSSDisplayTableRowGroup,
    CSSDisplayTableHeaderGroup,
    CSSDisplayTableFooterGroup,
    CSSDisplayTableRow,
    CSSDisplayTableColumnGroup,
    CSSDisplayTableColumn,
    CSSDisplayTableCell,
    CSSDisplayTableCaption,
    CSSDisplayNone
}

#[deriving_eq]
pub enum CSSPosition {
    CSSPositionStatic,
    CSSPositionRelative,
    CSSPositionAbsolute,
    CSSPositionFixed
}

pub enum CSSTop {
    CSSTopLength(Length),
    CSSTopPercentage,
    CSSTopAuto
}

pub enum CSSRight {
    CSSRightLength(Length),
    CSSRightPercentage(float),
    CSSRightAuto
}

pub enum CSSBottom {
    CSSBottomLength(Length),
    CSSBottomPercentage(float),
    CSSBottomAuto
}

pub enum CSSLeft {
    CSSLeftLength(Length),
    CSSLeftPercentage(float),
    CSSLeftAuto
}

#[deriving_eq]
pub enum CSSFloat {
    CSSFloatLeft,
    CSSFloatRight,
    CSSFloatNone
}

pub enum CSSDirection {
    CSSDirectionLtr,
    CSSDirectionRtl
}

// CSS 2.1, Section 10 - Visual formatting model details

#[deriving_eq]
pub enum CSSWidth {
    CSSWidthLength(Length),
    CSSWidthPercentage(float),
    CSSWidthAuto
}

#[deriving_eq]
pub enum CSSHeight {
    CSSHeightLength(Length),
    CSSHeightPercentage(float),
    CSSHeightAuto
}

#[deriving_eq]
pub enum CSSLineHeight {
    CSSLineHeightNormal,
    CSSLineHeightNumber(float),
    CSSLineHeightLength(Length),
    CSSLineHeightPercentage(float),
}

pub enum CSSVerticalAlign {
    CSSVerticalAlignBaseline,
    CSSVerticalAlignSub,
    CSSVerticalAlignSuper,
    CSSVerticalAlignTop,
    CSSVerticalAlignTextTop,
    CSSVerticalAlignMiddle,
    CSSVerticalAlignBottom,
    CSSVerticalAlignTextBottom,
    CSSVerticalAlignPercentage(float),
    CSSVerticalAlignLength(Length),
}

// CSS 2.1, Section 11 - Visual effects

pub enum CSSOverflow {
    CSSOverflowVisible,
    CSSOverflowHidden,
    CSSOverflowScroll,
    CSSOverflowAuto
}

pub enum CSSVisibility {
    CSSVisibilityVisible,
    CSSVisibilityHidden,
    CSSVisibilityCollapse
}

// CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

// CSS 2.1, Section 13 - Paged media

// CSS 2.1, Section 14 - Colors and Backgrounds

#[deriving_eq]
pub enum CSSColor {
    CSSColorColor(Color)
}

#[deriving_eq]
pub enum CSSBackgroundColor {
    CSSBackgroundColorColor(Color),
    CSSBackgroundColorTransparent
}

pub enum CSSBackgroundImage {
    CSSBackgroundUri(Url),
    CSSBackgroundImageNone
}

pub enum CSSBackgroundRepeat {
    CSSBackgroundRepeatRepeat,
    CSSBackgroundRepeatRepeatX,
    CSSBackgroundRepeatRepeatY,
    CSSBackgroundRepeatNoRepeat
}

pub enum CSSBackgroundAttachment {
    CSSBackgroundAttachmentScroll,
    CSSBackgroundAttachmentFixed
}

pub enum CSSBackgroundPosition {
    CSSBackgroundPositionPercentage(float),
    CSSBackgroundPositionLength(Length),
    CSSBackgroundPositionLeft,
    CSSBackgroundPositionCenter,
    CSSBackgroundPositionRight,
    CSSBackgroundPositionTop,
    CSSBackgroundPositionBottom
}

// CSS 2.1, Section 15 - Fonts

#[deriving_eq]
pub enum CSSFontFamily {
    CSSFontFamilyFamilyName(~str),
    CSSFontFamilyGenericFamily(GenericFontFamily)
}

#[deriving_eq]
pub enum CSSFontStyle {
    CSSFontStyleNormal,
    CSSFontStyleItalic,
    CSSFontStyleOblique
}

#[deriving_eq]
pub enum CSSFontWeight {
    CSSFontWeightNormal,
    CSSFontWeightBold,
    CSSFontWeightBolder,
    CSSFontWeightLighter,
    CSSFontWeight100,
    CSSFontWeight200,
    CSSFontWeight300,
    CSSFontWeight400,
    CSSFontWeight500,
    CSSFontWeight600,
    CSSFontWeight700,
    CSSFontWeight800,
    CSSFontWeight900
}

#[deriving_eq]
pub enum CSSFontSize {
    CSSFontSizeAbsoluteSize(AbsoluteSize),
    CSSFontSizeRelativeSize(RelativeSize),
    CSSFontSizeLength(Length),
    CSSFontSizePercentage(float)
}

// CSS 2.1, Section 16 - Text

#[deriving_eq]
pub enum CSSTextAlign {
    CSSTextAlignLeft,
    CSSTextAlignRight,
    CSSTextAlignCenter,
    CSSTextAlignJustify
}

pub enum CSSTextDecoration {
    CSSTextDecorationNone,
    CSSTextDecorationUnderline,
    CSSTextDecorationOverline,
    CSSTextDecorationLineThrough,
    CSSTextDecorationBlink
}

pub enum CSSTextTransform {
    CSSTextTransformCapitalize,
    CSSTextTransformUppercase,
    CSSTextTransformLowercase,
    CSSTextTransformNone
}

// CSS 2.1, Section 17 - Tables

// CSS 2.1, Section 18 - User interface

