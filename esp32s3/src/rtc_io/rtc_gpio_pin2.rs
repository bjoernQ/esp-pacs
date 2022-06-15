#[doc = "Register `RTC_GPIO_PIN2` reader"]
pub struct R(crate::R<RTC_GPIO_PIN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_PIN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_PIN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_PIN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_PIN2` writer"]
pub struct W(crate::W<RTC_GPIO_PIN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_PIN2_SPEC>;
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
impl From<crate::W<RTC_GPIO_PIN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_PIN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_DRIVER` reader - if set to 0: normal output, if set to 1: open drain"]
pub type PAD_DRIVER_R = crate::BitReader<bool>;
#[doc = "Field `PAD_DRIVER` writer - if set to 0: normal output, if set to 1: open drain"]
pub type PAD_DRIVER_W<'a> = crate::BitWriter<'a, u32, RTC_GPIO_PIN2_SPEC, bool, 2>;
#[doc = "Field `INT_TYPE` reader - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
pub type INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_TYPE` writer - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
pub type INT_TYPE_W<'a> = crate::FieldWriter<'a, u32, RTC_GPIO_PIN2_SPEC, u8, u8, 3, 7>;
#[doc = "Field `WAKEUP_ENABLE` reader - RTC GPIO wakeup enable bit"]
pub type WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_ENABLE` writer - RTC GPIO wakeup enable bit"]
pub type WAKEUP_ENABLE_W<'a> = crate::BitWriter<'a, u32, RTC_GPIO_PIN2_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 2 - if set to 0: normal output, if set to 1: open drain"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - RTC GPIO wakeup enable bit"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - if set to 0: normal output, if set to 1: open drain"]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W {
        PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W {
        INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - RTC GPIO wakeup enable bit"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W {
        WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC GPIO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_pin2](index.html) module"]
pub struct RTC_GPIO_PIN2_SPEC;
impl crate::RegisterSpec for RTC_GPIO_PIN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_pin2::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin2::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_PIN2 to value 0"]
impl crate::Resettable for RTC_GPIO_PIN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
