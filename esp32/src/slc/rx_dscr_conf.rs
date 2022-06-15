#[doc = "Register `RX_DSCR_CONF` reader"]
pub struct R(crate::R<RX_DSCR_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DSCR_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DSCR_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DSCR_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DSCR_CONF` writer"]
pub struct W(crate::W<RX_DSCR_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DSCR_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RX_DSCR_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DSCR_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` reader - "]
pub type SLC0_TOKEN_NO_REPLACE_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` writer - "]
pub type SLC0_TOKEN_NO_REPLACE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 0>;
#[doc = "Field `SLC0_INFOR_NO_REPLACE` reader - "]
pub type SLC0_INFOR_NO_REPLACE_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_INFOR_NO_REPLACE` writer - "]
pub type SLC0_INFOR_NO_REPLACE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 1>;
#[doc = "Field `SLC0_RX_FILL_MODE` reader - "]
pub type SLC0_RX_FILL_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_FILL_MODE` writer - "]
pub type SLC0_RX_FILL_MODE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 2>;
#[doc = "Field `SLC0_RX_EOF_MODE` reader - "]
pub type SLC0_RX_EOF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_EOF_MODE` writer - "]
pub type SLC0_RX_EOF_MODE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 3>;
#[doc = "Field `SLC0_RX_FILL_EN` reader - "]
pub type SLC0_RX_FILL_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_FILL_EN` writer - "]
pub type SLC0_RX_FILL_EN_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 4>;
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` reader - "]
pub type SLC0_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` writer - "]
pub type SLC0_RD_RETRY_THRESHOLD_W<'a> =
    crate::FieldWriter<'a, u32, RX_DSCR_CONF_SPEC, u16, u16, 11, 5>;
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` reader - "]
pub type SLC1_TOKEN_NO_REPLACE_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` writer - "]
pub type SLC1_TOKEN_NO_REPLACE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 16>;
#[doc = "Field `SLC1_INFOR_NO_REPLACE` reader - "]
pub type SLC1_INFOR_NO_REPLACE_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_INFOR_NO_REPLACE` writer - "]
pub type SLC1_INFOR_NO_REPLACE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 17>;
#[doc = "Field `SLC1_RX_FILL_MODE` reader - "]
pub type SLC1_RX_FILL_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_FILL_MODE` writer - "]
pub type SLC1_RX_FILL_MODE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 18>;
#[doc = "Field `SLC1_RX_EOF_MODE` reader - "]
pub type SLC1_RX_EOF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_EOF_MODE` writer - "]
pub type SLC1_RX_EOF_MODE_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 19>;
#[doc = "Field `SLC1_RX_FILL_EN` reader - "]
pub type SLC1_RX_FILL_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_FILL_EN` writer - "]
pub type SLC1_RX_FILL_EN_W<'a> = crate::BitWriter<'a, u32, RX_DSCR_CONF_SPEC, bool, 20>;
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` reader - "]
pub type SLC1_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` writer - "]
pub type SLC1_RD_RETRY_THRESHOLD_W<'a> =
    crate::FieldWriter<'a, u32, RX_DSCR_CONF_SPEC, u16, u16, 11, 21>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&self) -> SLC0_TOKEN_NO_REPLACE_R {
        SLC0_TOKEN_NO_REPLACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&self) -> SLC0_INFOR_NO_REPLACE_R {
        SLC0_INFOR_NO_REPLACE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&self) -> SLC0_RX_FILL_MODE_R {
        SLC0_RX_FILL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&self) -> SLC0_RX_EOF_MODE_R {
        SLC0_RX_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&self) -> SLC0_RX_FILL_EN_R {
        SLC0_RX_FILL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&self) -> SLC0_RD_RETRY_THRESHOLD_R {
        SLC0_RD_RETRY_THRESHOLD_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&self) -> SLC1_TOKEN_NO_REPLACE_R {
        SLC1_TOKEN_NO_REPLACE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&self) -> SLC1_INFOR_NO_REPLACE_R {
        SLC1_INFOR_NO_REPLACE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&self) -> SLC1_RX_FILL_MODE_R {
        SLC1_RX_FILL_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&self) -> SLC1_RX_EOF_MODE_R {
        SLC1_RX_EOF_MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&self) -> SLC1_RX_FILL_EN_R {
        SLC1_RX_FILL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&self) -> SLC1_RD_RETRY_THRESHOLD_R {
        SLC1_RD_RETRY_THRESHOLD_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&mut self) -> SLC0_TOKEN_NO_REPLACE_W {
        SLC0_TOKEN_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&mut self) -> SLC0_INFOR_NO_REPLACE_W {
        SLC0_INFOR_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&mut self) -> SLC0_RX_FILL_MODE_W {
        SLC0_RX_FILL_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&mut self) -> SLC0_RX_EOF_MODE_W {
        SLC0_RX_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&mut self) -> SLC0_RX_FILL_EN_W {
        SLC0_RX_FILL_EN_W::new(self)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&mut self) -> SLC0_RD_RETRY_THRESHOLD_W {
        SLC0_RD_RETRY_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&mut self) -> SLC1_TOKEN_NO_REPLACE_W {
        SLC1_TOKEN_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&mut self) -> SLC1_INFOR_NO_REPLACE_W {
        SLC1_INFOR_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&mut self) -> SLC1_RX_FILL_MODE_W {
        SLC1_RX_FILL_MODE_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&mut self) -> SLC1_RX_EOF_MODE_W {
        SLC1_RX_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&mut self) -> SLC1_RX_FILL_EN_W {
        SLC1_RX_FILL_EN_W::new(self)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&mut self) -> SLC1_RD_RETRY_THRESHOLD_W {
        SLC1_RD_RETRY_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_dscr_conf](index.html) module"]
pub struct RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_dscr_conf::R](R) reader structure"]
impl crate::Readable for RX_DSCR_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_dscr_conf::W](W) writer structure"]
impl crate::Writable for RX_DSCR_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_DSCR_CONF to value 0x101b_101a"]
impl crate::Resettable for RX_DSCR_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x101b_101a
    }
}
