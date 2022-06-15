#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_RESET` reader - "]
pub type TX_RESET_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESET` writer - "]
pub type TX_RESET_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 0>;
#[doc = "Field `RX_RESET` reader - "]
pub type RX_RESET_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESET` writer - "]
pub type RX_RESET_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 1>;
#[doc = "Field `TX_FIFO_RESET` reader - "]
pub type TX_FIFO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_RESET` writer - "]
pub type TX_FIFO_RESET_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 2>;
#[doc = "Field `RX_FIFO_RESET` reader - "]
pub type RX_FIFO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_RESET` writer - "]
pub type RX_FIFO_RESET_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 3>;
#[doc = "Field `TX_START` reader - "]
pub type TX_START_R = crate::BitReader<bool>;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 4>;
#[doc = "Field `RX_START` reader - "]
pub type RX_START_R = crate::BitReader<bool>;
#[doc = "Field `RX_START` writer - "]
pub type RX_START_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 5>;
#[doc = "Field `TX_SLAVE_MOD` reader - "]
pub type TX_SLAVE_MOD_R = crate::BitReader<bool>;
#[doc = "Field `TX_SLAVE_MOD` writer - "]
pub type TX_SLAVE_MOD_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 6>;
#[doc = "Field `RX_SLAVE_MOD` reader - "]
pub type RX_SLAVE_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RX_SLAVE_MOD` writer - "]
pub type RX_SLAVE_MOD_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 7>;
#[doc = "Field `TX_RIGHT_FIRST` reader - "]
pub type TX_RIGHT_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `TX_RIGHT_FIRST` writer - "]
pub type TX_RIGHT_FIRST_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 8>;
#[doc = "Field `RX_RIGHT_FIRST` reader - "]
pub type RX_RIGHT_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `RX_RIGHT_FIRST` writer - "]
pub type RX_RIGHT_FIRST_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 9>;
#[doc = "Field `TX_MSB_SHIFT` reader - "]
pub type TX_MSB_SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `TX_MSB_SHIFT` writer - "]
pub type TX_MSB_SHIFT_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 10>;
#[doc = "Field `RX_MSB_SHIFT` reader - "]
pub type RX_MSB_SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `RX_MSB_SHIFT` writer - "]
pub type RX_MSB_SHIFT_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 11>;
#[doc = "Field `TX_SHORT_SYNC` reader - "]
pub type TX_SHORT_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `TX_SHORT_SYNC` writer - "]
pub type TX_SHORT_SYNC_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 12>;
#[doc = "Field `RX_SHORT_SYNC` reader - "]
pub type RX_SHORT_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `RX_SHORT_SYNC` writer - "]
pub type RX_SHORT_SYNC_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 13>;
#[doc = "Field `TX_MONO` reader - "]
pub type TX_MONO_R = crate::BitReader<bool>;
#[doc = "Field `TX_MONO` writer - "]
pub type TX_MONO_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 14>;
#[doc = "Field `RX_MONO` reader - "]
pub type RX_MONO_R = crate::BitReader<bool>;
#[doc = "Field `RX_MONO` writer - "]
pub type RX_MONO_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 15>;
#[doc = "Field `TX_MSB_RIGHT` reader - "]
pub type TX_MSB_RIGHT_R = crate::BitReader<bool>;
#[doc = "Field `TX_MSB_RIGHT` writer - "]
pub type TX_MSB_RIGHT_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 16>;
#[doc = "Field `RX_MSB_RIGHT` reader - "]
pub type RX_MSB_RIGHT_R = crate::BitReader<bool>;
#[doc = "Field `RX_MSB_RIGHT` writer - "]
pub type RX_MSB_RIGHT_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 17>;
#[doc = "Field `SIG_LOOPBACK` reader - "]
pub type SIG_LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `SIG_LOOPBACK` writer - "]
pub type SIG_LOOPBACK_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_reset(&self) -> TX_RESET_R {
        TX_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_reset(&self) -> RX_RESET_R {
        RX_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_reset(&self) -> TX_FIFO_RESET_R {
        TX_FIFO_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_reset(&self) -> RX_FIFO_RESET_R {
        RX_FIFO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_right_first(&self) -> TX_RIGHT_FIRST_R {
        TX_RIGHT_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_right_first(&self) -> RX_RIGHT_FIRST_R {
        RX_RIGHT_FIRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> TX_MSB_SHIFT_R {
        TX_MSB_SHIFT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RX_MSB_SHIFT_R {
        RX_MSB_SHIFT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_short_sync(&self) -> TX_SHORT_SYNC_R {
        TX_SHORT_SYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rx_short_sync(&self) -> RX_SHORT_SYNC_R {
        RX_SHORT_SYNC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_msb_right(&self) -> TX_MSB_RIGHT_R {
        TX_MSB_RIGHT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_msb_right(&self) -> RX_MSB_RIGHT_R {
        RX_MSB_RIGHT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_reset(&mut self) -> TX_RESET_W {
        TX_RESET_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RX_RESET_W {
        RX_RESET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W {
        TX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W {
        RX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W {
        RX_START_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W {
        TX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W {
        RX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_right_first(&mut self) -> TX_RIGHT_FIRST_W {
        TX_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_right_first(&mut self) -> RX_RIGHT_FIRST_W {
        RX_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W {
        TX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W {
        RX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_short_sync(&mut self) -> TX_SHORT_SYNC_W {
        TX_SHORT_SYNC_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rx_short_sync(&mut self) -> RX_SHORT_SYNC_W {
        RX_SHORT_SYNC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_mono(&mut self) -> TX_MONO_W {
        TX_MONO_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_mono(&mut self) -> RX_MONO_W {
        RX_MONO_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_msb_right(&mut self) -> TX_MSB_RIGHT_W {
        TX_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_msb_right(&mut self) -> RX_MSB_RIGHT_W {
        RX_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W {
        SIG_LOOPBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0x0003_0300"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0300
    }
}
