#[doc = "Register `SWFC_CONF0` reader"]
pub type R = crate::R<SWFC_CONF0_SPEC>;
#[doc = "Register `SWFC_CONF0` writer"]
pub type W = crate::W<SWFC_CONF0_SPEC>;
#[doc = "Field `XOFF_THRESHOLD` reader - When the number of data bytes in RX FIFO is more than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XOFF character."]
pub type XOFF_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `XOFF_THRESHOLD` writer - When the number of data bytes in RX FIFO is more than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XOFF character."]
pub type XOFF_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the XOFF flow control character."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the XOFF flow control character."]
pub type XOFF_CHAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:8 - When the number of data bytes in RX FIFO is more than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XOFF character."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - This register stores the XOFF flow control character."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF0")
            .field(
                "xoff_threshold",
                &format_args!("{}", self.xoff_threshold().bits()),
            )
            .field("xoff_char", &format_args!("{}", self.xoff_char().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWFC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - When the number of data bytes in RX FIFO is more than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XOFF character."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W<SWFC_CONF0_SPEC, 0> {
        XOFF_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 9:16 - This register stores the XOFF flow control character."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<SWFC_CONF0_SPEC, 9> {
        XOFF_CHAR_W::new(self)
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
#[doc = "Software flow control character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWFC_CONF0_SPEC;
impl crate::RegisterSpec for SWFC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf0::R`](R) reader structure"]
impl crate::Readable for SWFC_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf0::W`](W) writer structure"]
impl crate::Writable for SWFC_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWFC_CONF0 to value 0x26e0"]
impl crate::Resettable for SWFC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x26e0;
}
