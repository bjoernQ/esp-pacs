#[doc = "Register `LCD_CTRL1` reader"]
pub type R = crate::R<LCD_CTRL1_SPEC>;
#[doc = "Register `LCD_CTRL1` writer"]
pub type W = crate::W<LCD_CTRL1_SPEC>;
#[doc = "Field `LCD_VB_FRONT` reader - It is the vertical blank front porch of a frame. Can be configured in CONF state."]
pub type LCD_VB_FRONT_R = crate::FieldReader;
#[doc = "Field `LCD_VB_FRONT` writer - It is the vertical blank front porch of a frame. Can be configured in CONF state."]
pub type LCD_VB_FRONT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LCD_HA_WIDTH` reader - It is the horizontal active width of a frame. Can be configured in CONF state."]
pub type LCD_HA_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HA_WIDTH` writer - It is the horizontal active width of a frame. Can be configured in CONF state."]
pub type LCD_HA_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `LCD_HT_WIDTH` reader - It is the horizontal total width of a frame. Can be configured in CONF state."]
pub type LCD_HT_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HT_WIDTH` writer - It is the horizontal total width of a frame. Can be configured in CONF state."]
pub type LCD_HT_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:7 - It is the vertical blank front porch of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_vb_front(&self) -> LCD_VB_FRONT_R {
        LCD_VB_FRONT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - It is the horizontal active width of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_ha_width(&self) -> LCD_HA_WIDTH_R {
        LCD_HA_WIDTH_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - It is the horizontal total width of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_ht_width(&self) -> LCD_HT_WIDTH_R {
        LCD_HT_WIDTH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CTRL1")
            .field(
                "lcd_vb_front",
                &format_args!("{}", self.lcd_vb_front().bits()),
            )
            .field(
                "lcd_ha_width",
                &format_args!("{}", self.lcd_ha_width().bits()),
            )
            .field(
                "lcd_ht_width",
                &format_args!("{}", self.lcd_ht_width().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - It is the vertical blank front porch of a frame. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vb_front(&mut self) -> LCD_VB_FRONT_W<LCD_CTRL1_SPEC, 0> {
        LCD_VB_FRONT_W::new(self)
    }
    #[doc = "Bits 8:19 - It is the horizontal active width of a frame. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_ha_width(&mut self) -> LCD_HA_WIDTH_W<LCD_CTRL1_SPEC, 8> {
        LCD_HA_WIDTH_W::new(self)
    }
    #[doc = "Bits 20:31 - It is the horizontal total width of a frame. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_ht_width(&mut self) -> LCD_HT_WIDTH_W<LCD_CTRL1_SPEC, 20> {
        LCD_HT_WIDTH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LCD frame control1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CTRL1_SPEC;
impl crate::RegisterSpec for LCD_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ctrl1::R`](R) reader structure"]
impl crate::Readable for LCD_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ctrl1::W`](W) writer structure"]
impl crate::Writable for LCD_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_CTRL1 to value 0"]
impl crate::Resettable for LCD_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
